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
    "./artifacts/TPFt.json"
);

#[tokio::main]
pub async fn create<P: Provider>(
    provider: &P,
    contract_address: Address,
    acronym: String,
    code: String,
    maturity_date: u64,
) -> Result<TransactionReceipt> {
    let tpft = TPFt::new(contract_address, provider);
    println!("TPFt address: {}", tpft.address());

    // Call the createTPFt function on the contract.
    let tx = tpft.createTPFt(acronym, code, Uint::from(maturity_date));
    let pending_tx = tx.send().await?;
    let receipt = pending_tx.get_receipt().await?;
    println!("TPFt create receipt: {:?}", receipt);
    Ok(receipt)
}
