use std::net::TcpListener;
use zero2prod::startup;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener: TcpListener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    startup::run(listener)?.await
}
