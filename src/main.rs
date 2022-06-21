use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration");

    // We have remove the hard-coded `8000` - it's now coming from our settings
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener: TcpListener = TcpListener::bind(address).expect("Failed to bind random port");
    startup::run(listener)?.await
}
