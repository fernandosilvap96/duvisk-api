//! Example of deploying a contract from an artifact using the `sol!` macro to Anvil and interacting
//! with it.

use alloy::{core::primitives::Address, providers::Provider, sol};
use eyre::Result;

// Codegen from artifact.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    TPFt,
    "lib/selic-contracts/out/TPFt.sol/TPFt.json"
);

#[tokio::main]
pub async fn deploy<P: Provider>(provider: &P, public_key: Address) -> Result<Address> {
    // Deploy the `Counter` contract.
    let contract = TPFt::deploy(&provider, public_key).await?;
    println!(
        "Deployed contract at address: {}",
        contract.address().clone()
    );

    Ok(*contract.address())
}
