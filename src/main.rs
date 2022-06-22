use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration");

    // We have remove the hard-coded `8000` - it's now coming from our settings
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let listener: TcpListener = TcpListener::bind(address).expect("Failed to bind random port");
    startup::run(listener, connection_pool)?.await
}
