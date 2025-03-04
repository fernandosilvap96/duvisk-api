//! Example of deploying a contract from an artifact using the `sol!` macro to Anvil and interacting
//! with it.

use alloy::{core::primitives::Address, core::primitives::Uint, providers::Provider, sol};
use eyre::Result;

// Codegen from artifact.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug)]
    TPFt,
    "lib/selic-contracts/out/TPFt.sol/TPFt.json"
);

pub async fn get<P: Provider>(
    provider: &P,
    contract_address: Address,
    tpft_id: u64,
) -> Result<TPFt::getTPFtDataReturn> {
    let tpft = TPFt::new(contract_address, provider);
    println!("TPFt address: {}", tpft.address());

    // Call the createTPFt function on the contract.
    let tx = tpft.getTPFtData(Uint::from(tpft_id));
    let tpft_data = tx.call().await?;
    println!("TPFt data: {:?}", tpft_data);
    Ok(tpft_data)
}
