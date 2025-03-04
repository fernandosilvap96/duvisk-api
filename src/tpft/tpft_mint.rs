//! Example of deploying a contract from an artifact using the `sol!` macro to Anvil and interacting
//! with it.

use alloy::{
    core::primitives::Address, core::primitives::Uint, providers::Provider,
    rpc::types::TransactionReceipt, sol,
};
use eyre::Result;

// Codegen from artifact.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    TPFt,
    "lib/selic-contracts/out/TPFt.sol/TPFt.json"
);

pub async fn mint<P: Provider>(
    provider: &P,
    contract_address: Address,
    receiver: Address,
    tpft_id: u64,
    amount: u64,
) -> Result<TransactionReceipt> {
    let tpft = TPFt::new(contract_address, provider);
    println!("TPFt address: {}", tpft.address());
    let tx = tpft.mint(receiver, Uint::from(tpft_id), Uint::from(amount));
    let pending_tx = tx.send().await?;
    let receipt = pending_tx.get_receipt().await?;
    println!("TPFt mint receipt: {:?}", receipt);
    Ok(receipt)
}
