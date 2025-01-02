use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let address = "127.0.0.1:8000";
    let listener = TcpListener::bind(address).expect("Failed to bind random port.");
    run(listener)?.await
}
