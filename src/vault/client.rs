use crate::configuration::vault::VaultConfiguration;
use crate::utilities::watcher::Refresh;
use crate::vault::keys::KeyInfo;
use std::{fs, string::ToString};
use tonic::{async_trait, Code, Status};
use tracing::{debug, instrument, warn};
use vaultrs::client::{Client as ClientTrait, VaultClient};
use vaultrs::{api::AuthInfo, error::ClientError, transit};
use crate::configuration::authentication::{Authentication, Credentials};

const TRANSIT_MOUNT: &str = "transit";
const KUBERNETES_AUTH_MOUNT: &str = "kubernetes";

#[derive(Debug)]
pub struct VaultError(pub ClientError);

impl From<VaultError> for Status {
    fn from(value: VaultError) -> Self {
        Status::new(Code::Internal, value.0.to_string())
    }
}

impl From<ClientError> for VaultError {
    fn from(value: ClientError) -> Self {
        VaultError(value)
    }
}

pub struct Client {
    pub key_name: String,
    role: String,
    auth: Authentication,
    client: VaultClient,
}

#[async_trait]
impl Refresh for Client {
    #[instrument(skip(self))]
    async fn refresh_token(&mut self) -> Result<(), std::io::Error> {
        let token = self
            .get_token()
            .await
            .map_err(|error| std::io::Error::other(error.to_string()))?;
        self.client.set_token(&token);
        Ok(())
    }
}

impl Client {
    #[instrument(skip(self, path))]
    async fn request_token_with_jwt(&self, path: &str) -> Result<String, ClientError> {
        let jwt = fs::read_to_string(path).map_err(|error| ClientError::FileReadError {
            source: error,
            path: path.to_string(),
        })?;
        debug!("Logging in via JWT: {}", jwt.len());
        Ok(self.jwt_auth(&jwt).await?.client_token)
    }

    #[instrument(skip(self))]
    pub async fn get_token(&self) -> Result<String, ClientError> {
        match self.auth.clone() {
            Authentication::Token(token) => Ok(token),
            Authentication::Kubernetes(path) => Ok(self.request_token_with_jwt(&path).await?),
            Authentication::Credentials(credentials) => Ok(self.request_token_with_credentials(&credentials).await?.client_token),
            Authentication::None => Err(ClientError::APIError {
                code: 500,
                errors: vec!["No token found".to_string()],
            }),
        }
    }

    pub fn new(client: VaultClient, config: &VaultConfiguration) -> Self {
        Self {
            role: config.role.to_string(),
            key_name: config.transit_key.to_string(),
            auth: config.auth.clone(),
            client,
        }
    }

    #[instrument(skip(self, credentials))]
    pub async fn request_token_with_credentials(&self, credentials: &Credentials) -> Result<AuthInfo, ClientError> {
        debug!("Logging in with credentials: {:?}", credentials);
        Ok(vaultrs::auth::userpass::login(&self.client, &credentials.path, &credentials.username, &credentials.password).await?)
    }

    #[instrument(skip(self, jwt))]
    pub async fn jwt_auth(&self, jwt: &str) -> Result<AuthInfo, ClientError> {
        debug!("Logging in to vault as: {}", self.role);
        Ok(
            vaultrs::auth::kubernetes::login(&self.client, KUBERNETES_AUTH_MOUNT, &self.role, &jwt)
                .await?,
        )
    }

    #[instrument(skip(self))]
    pub async fn request_key(&self) -> Result<KeyInfo, VaultError> {
        Ok(
            transit::key::read(&self.client, TRANSIT_MOUNT, &self.key_name)
                .await?
                .keys
                .into(),
        )
    }

    #[instrument(skip(self, data))]
    pub async fn request_encryption(&self, data: &str) -> Result<String, VaultError> {
        debug!("Requesting encryption, data: {}", data);
        Ok(
            transit::data::encrypt(&self.client, TRANSIT_MOUNT, &self.key_name, data, None)
                .await?
                .ciphertext,
        )
    }

    #[instrument(skip(self, data))]
    pub async fn request_decryption(&self, data: &str) -> Result<String, VaultError> {
        debug!("Requesting decryption, data: {}", data);
        Ok(
            transit::data::decrypt(&self.client, TRANSIT_MOUNT, &self.key_name, data, None)
                .await?
                .plaintext,
        )
    }

    #[instrument(skip(self, token))]
    pub fn set_token(&mut self, token: &str) -> () {
        debug!("Setting token: {}", token);
        self.client.set_token(token);
    }
}
