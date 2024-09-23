use crate::configuration::vault::VaultConfiguration;
use crate::vault::keys::KeyInfo;
use std::{string::ToString, sync::Arc, sync::atomic::{AtomicBool}};
use tonic::{Code, Status};
use tracing::{debug, instrument};
use vaultrs::{api::AuthInfo, error::ClientError, transit, client::{VaultClient}};

const TRANSIT_MOUNT: &str = "transit";
const KUBERNETES_AUTH_MOUNT: &str = "kubernetes";

#[derive(Debug)]
pub struct VaultError(pub(crate) ClientError);

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
  role: String,
  pub(crate) key_name: String,
  pub(crate) token: Option<String>,
  pub(crate) jwt_path: Option<String>,
  pub(crate) client: VaultClient,
  pub(crate) rotate: Arc<AtomicBool>
}

impl Client{
  pub fn new(client: VaultClient, config: &VaultConfiguration, rotate: Arc<AtomicBool>) -> Self {
    Self {
      role: config.vault_role.to_string(),
      key_name: config.vault_transit_key.to_string(),
      jwt_path: config.jwt_path.clone(),
      token: config.vault_token.clone(),
      rotate,
      client,
    }
  }

  #[instrument(skip(self, jwt))]
  pub(crate) async fn jwt_auth(&self, jwt: &str) -> Result<AuthInfo, ClientError> {
    debug!("Logging in to vault as: {}", self.role);
    Ok(vaultrs::auth::kubernetes::login(&self.client, KUBERNETES_AUTH_MOUNT, &self.role, &jwt).await?)
  }

  #[instrument(skip(self))]
  pub(crate) async fn request_key(&self) -> Result<KeyInfo, VaultError> {
    Ok(
      transit::key::read(&self.client, TRANSIT_MOUNT, &self.key_name)
        .await?
        .keys
        .into(),
    )
  }

  #[instrument(skip(self, data))]
  pub(crate) async fn request_encryption(&self, data: &str) -> Result<String, VaultError> {
    debug!("Requesting encryption, data: {}", data);
    Ok(
      transit::data::encrypt(&self.client, TRANSIT_MOUNT, &self.key_name, data, None)
        .await?
        .ciphertext,
    )
  }

  #[instrument(skip(self, data))]
  pub(crate) async fn request_decryption(&self, data: &str) -> Result<String, VaultError> {
    debug!("Requesting decryption, data: {}", data);
    Ok(
      transit::data::decrypt(&self.client, TRANSIT_MOUNT, &self.key_name, data, None)
        .await?
        .plaintext,
    )
  }
}
