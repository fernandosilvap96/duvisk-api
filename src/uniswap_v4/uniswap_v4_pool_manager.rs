//! Example of deploying a contract from an artifact using the `sol!` macro to Anvil and interacting
//! with it.
use alloy::{core::primitives::Address, providers::Provider, sol};
use eyre::Result;

// Codegen from artifact.
sol!(
    #[allow(missing_docs)]
    #[derive(Debug)]
    #[sol(rpc)]
    PoolManager,
    "lib/selic-contracts/out/PoolManager.sol/PoolManager.json",
);

pub async fn get_owner<P: Provider>(provider: &P, pool_manager: Address) -> Result<Address> {
    let pool_manager = PoolManager::new(pool_manager, provider);
    let tx = pool_manager.owner();
    let owner = tx.call().await?;
    println!("PoolManager owner: {:?}", &owner._0);
    Ok(owner._0)
}
