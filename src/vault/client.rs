use crate::configuration::authentication::{
    AppRole, Certificate, Credentials, Jwt, Kubernetes, UserPass,
};
use crate::configuration::vault::VaultConfiguration;
use crate::utilities::watcher::Refresh;
use crate::vault::keys::KeyInfo;
use std::string::ToString;
use tonic::{async_trait, Code, Status};
use tracing::{debug, instrument, warn};
use vaultrs::api::transit::responses::ReadKeyData;
use vaultrs::client::{Client as ClientTrait, VaultClient};
use vaultrs::{api::AuthInfo, error::ClientError, transit};

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
    key_name: String,
    auth: Credentials,
    client: VaultClient,
    mount_path: String,
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
    #[instrument(skip(self, credentials))]
    async fn kubernetes_authentication(
        &self,
        credentials: &Kubernetes,
    ) -> Result<AuthInfo, ClientError> {
        debug!("Logging in with kubernetes auth: {:?}", credentials);
        Ok(vaultrs::auth::kubernetes::login(
            &self.client,
            &credentials.mount_path,
            &credentials.role,
            &credentials.jwt.value()?,
        )
        .await?)
    }

    #[instrument(skip(self, credentials))]
    async fn jwt_authentication(&self, credentials: &Jwt) -> Result<AuthInfo, ClientError> {
        debug!("Logging in with JWT authentication: {:?}", credentials);
        Ok(vaultrs::auth::oidc::login(
            &self.client,
            &credentials.mount_path,
            &credentials.jwt.value()?,
            credentials.role.clone(),
        )
        .await?)
    }

    #[instrument(skip(self, credentials))]
    async fn cert_authentication(
        &self,
        credentials: &Certificate,
    ) -> Result<AuthInfo, ClientError> {
        debug!("Logging in with JWT authentication: {:?}", credentials);
        Ok(
            vaultrs::auth::cert::login(&self.client, &credentials.mount_path, &credentials.name)
                .await?,
        )
    }

    #[instrument(skip(self))]
    pub async fn get_token(&self) -> Result<String, ClientError> {
        match &self.auth {
            Credentials::Token(token) => Ok(token.value()?),
            Credentials::Kubernetes(credentials) => Ok(self
                .kubernetes_authentication(credentials)
                .await?
                .client_token),
            Credentials::UserPass(credentials) => Ok(self
                .user_pass_authentication(credentials)
                .await?
                .client_token),
            Credentials::AppRole(credentials) => Ok(self
                .app_role_authentication(credentials)
                .await?
                .client_token),
            Credentials::Jwt(jwt) => Ok(self.jwt_authentication(jwt).await?.client_token),
            Credentials::Certificate(credentials) => {
                Ok(self.cert_authentication(credentials).await?.client_token)
            }
            Credentials::None => Err(ClientError::APIError {
                code: 500,
                errors: vec!["No token found".to_string()],
            }),
        }
    }

    pub fn new(client: VaultClient, config: &VaultConfiguration) -> Self {
        Self {
            key_name: config.transit_key.to_string(),
            auth: config.credentials.clone(),
            mount_path: config.mount_path.clone(),
            client,
        }
    }

    #[instrument(skip(self, credentials))]
    pub async fn user_pass_authentication(
        &self,
        credentials: &UserPass,
    ) -> Result<AuthInfo, ClientError> {
        debug!("Logging in with UserPass credentials: {:?}", credentials);
        Ok(vaultrs::auth::userpass::login(
            &self.client,
            &credentials.mount_path,
            &credentials.username,
            &credentials.password.value()?,
        )
        .await?)
    }

    #[instrument(skip(self, credentials))]
    pub async fn app_role_authentication(
        &self,
        credentials: &AppRole,
    ) -> Result<AuthInfo, ClientError> {
        debug!("Logging in with AppRole credentials: {:?}", credentials);
        Ok(vaultrs::auth::approle::login(
            &self.client,
            &credentials.mount_path,
            &credentials.role_id,
            &credentials.secret_id.value()?,
        )
        .await?)
    }

    #[instrument(skip(self))]
    pub async fn request_key(&self) -> Result<KeyInfo, VaultError> {
        Ok(
            match transit::key::read(&self.client, &self.mount_path, &self.key_name)
                .await?
                .keys
            {
                ReadKeyData::Asymmetric(data) => data.into(),
                ReadKeyData::Symmetric(data) => data.into(),
            },
        )
    }

    #[instrument(skip(self, data))]
    pub async fn request_encryption(&self, data: &str) -> Result<String, VaultError> {
        debug!("Requesting encryption, data: {}", data);
        Ok(
            transit::data::encrypt(&self.client, &self.mount_path, &self.key_name, data, None)
                .await?
                .ciphertext,
        )
    }

    #[instrument(skip(self, data))]
    pub async fn request_decryption(&self, data: &str) -> Result<String, VaultError> {
        debug!("Requesting decryption, data: {}", data);
        Ok(
            transit::data::decrypt(&self.client, &self.mount_path, &self.key_name, data, None)
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
