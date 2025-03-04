//! Example of deploying a contract from an artifact using the `sol!` macro to Anvil and interacting
//! with it.

use alloy::{core::primitives::Address, providers::Provider, sol};
use eyre::Result;

// Codegen from artifact.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20Mock,
    "lib/selic-contracts/out/ERC20Mock.sol/ERC20Mock.json"
);

pub async fn deploy<P: Provider>(provider: &P) -> Result<Address> {
    // Deploy the `Counter` contract.
    let contract = ERC20Mock::deploy(&provider).await?;
    println!(
        "Deployed ERC20Mock at address: {}",
        contract.address().clone()
    );

    Ok(*contract.address())
}
