use crate::configuration::vault::VaultConfiguration;
use crate::vault::keys::KeyInfo;
use std::{fs, string::ToString};
use tonic::{Code, Status};
use tracing::{debug, instrument, warn};
use vaultrs::client::Client as ClientExt;
use vaultrs::{api::AuthInfo, client::VaultClient, error::ClientError, transit};

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
    token: Option<String>,
    jwt_path: Option<String>,
    client: VaultClient,
}

impl Client {
    #[instrument(skip(self, path))]
    async fn request_token_with_jwt(&self, path: &str) -> Result<String, ClientError> {
        let jwt = fs::read_to_string(path).map_err(|_| ClientError::FileNotFoundError {
            path: path.to_string(),
        })?;
        debug!("Using mounted jwt of length: {}", jwt.len());
        Ok(self.jwt_auth(&jwt).await?.client_token)
    }

    #[instrument(skip(self))]
    async fn get_token(&self) -> Result<Option<String>, ClientError> {
        Ok(if let Some(path) = self.jwt_path.clone() {
            Some(self.request_token_with_jwt(&path).await?)
        } else if let Some(token) = self.token.clone() {
            Some(token)
        } else {
            warn!("No token found");
            None
        })
    }

    #[instrument(skip(self))]
    pub(crate) async fn refresh_token(&mut self) -> Result<(), ClientError> {
        if let Some(token) = self.get_token().await? {
            self.client.set_token(&token);
        }
        Ok(())
    }

    pub fn new(client: VaultClient, config: &VaultConfiguration) -> Self {
        Self {
            role: config.vault_role.to_string(),
            key_name: config.vault_transit_key.to_string(),
            jwt_path: config.jwt_path.clone(),
            token: config.vault_token.clone(),
            client,
        }
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
        self.client.set_token(token);
    }
}
