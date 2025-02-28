//! Example of deploying a contract from an artifact using the `sol!` macro to Anvil and interacting
//! with it.

use alloy::{providers::Provider, sol};
use eyre::Result;

// Codegen from artifact.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Counter,
    "./artifacts/Counter.json"
);

#[tokio::main]
pub async fn deploy<P: Provider>(provider: &P) -> Result<()> {
    // Deploy the `Counter` contract.
    let contract = Counter::deploy(&provider).await?;

    println!("Deployed contract at address: {}", contract.address());

    Ok(())
}
