//! Example of deploying a contract from an artifact using the `sol!` macro to Anvil and interacting
//! with it.

use crate::utils::math::Int24;
use crate::utils::math::Uint160;
use alloy::{core::primitives::Address, core::primitives::Uint, providers::Provider, sol};
use eyre::Result;

// Codegen from artifact.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    TPFtPoolManager,
    "lib/selic-contracts/out/TPFtPoolManager.sol/TPFtPoolManager.json"
);

pub async fn deploy<P: Provider>(
    provider: &P,
    tpft: Address,
    realdigital: Address,
    lp_fee: u64,
    tick_spacing: Int24,
    starting_price: Uint160,
) -> Result<Address> {
    // Deploy the `Counter` contract.
    let contract = TPFtPoolManager::deploy(
        &provider,
        tpft,
        realdigital,
        Uint::from(lp_fee),
        tick_spacing,
        starting_price,
    )
    .await?;
    println!(
        "Deployed PoolManager at address: {}",
        contract.address().clone()
    );

    Ok(*contract.address())
}
