extern crate lib;

use lib::utilities::logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::initialize();
    lib::server().await
}
