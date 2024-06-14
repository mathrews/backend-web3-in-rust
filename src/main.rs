use axum::{routing::get, Extension, Router};
use backend_web3_rust::{counter::Counter, routes};
use ethers::{
    contract::abigen, prelude::{Http, Provider}, types::Address
};
use eyre::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio;
use tracing::info;
use tracing_subscriber;

abigen!(CounterContract, "./src/counter.json");

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // Ethers-rs part for interacting with the contract
    let provider = Provider::<Http>::try_from("https://mainnet.infura.io/v3/6ffbbaba936b4bec8e9672261ae1e797").unwrap();
    let counter_address: Address =
        String::from("0x760f270f75b62ce2213ca982d89538b12574a547").parse()?;
    let counter = Counter::new(counter_address, Arc::new(provider));
    match counter.get_number().await {
        Ok(number) => println!("The number is: {}", number),
        Err(e) => println!("Error: {:?}", e),
    }

    // Axum server
    let app = Router::new()
        .route("/api/number/", get(routes::handle_number))
        .route("/api/block_number/", get(routes::handle_block_number))
        .layer(Extension(counter));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("LISTENING on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
