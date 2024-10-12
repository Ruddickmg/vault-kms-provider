pub mod authentication;
pub mod health;
pub mod logging;
pub mod socket;
pub mod tls;
pub mod vault;

#[derive(Clone, Debug)]
pub struct ServerConfiguration {
    pub socket: socket::SocketConfiguration,
    pub vault: vault::VaultConfiguration,
    pub tls: tls::TlsConfiguration,
    pub health: health::HealthCheckConfiguration,
}

impl Default for ServerConfiguration {
    fn default() -> Self {
        Self {
            socket: socket::SocketConfiguration::default(),
            vault: vault::VaultConfiguration::default(),
            tls: tls::TlsConfiguration::default(),
            health: health::HealthCheckConfiguration::default(),
        }
    }
}
