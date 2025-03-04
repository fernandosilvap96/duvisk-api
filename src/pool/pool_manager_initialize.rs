//! Example of deploying a contract from an artifact using the `sol!` macro to Anvil and interacting
//! with it.

use alloy::{core::primitives::Address, providers::Provider, rpc::types::TransactionReceipt, sol};
use eyre::Result;

// Codegen from artifact.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    TPFtPoolManager,
    "lib/selic-contracts/out/TPFtPoolManager.sol/TPFtPoolManager.json"
);

pub async fn initialize<P: Provider>(
    provider: &P,
    contract_address: Address,
) -> Result<TransactionReceipt> {
    // Deploy the `Counter` contract.
    let pool_manager = TPFtPoolManager::new(contract_address, provider);
    println!("PoolManager address: {}", pool_manager.address());

    let tx = pool_manager.initializePool();
    let pending_tx = tx.send().await?;
    let receipt = pending_tx.get_receipt().await?;
    println!("PoolManager initialize receipt: {:?}", receipt);
    Ok(receipt)
}
