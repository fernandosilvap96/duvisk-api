//! Example of deploying a contract from an artifact using the `sol!` macro to Anvil and interacting
//! with it.

use alloy::{
    core::primitives::Address, core::primitives::Uint, providers::Provider,
    rpc::types::TransactionReceipt, sol,
};
use alloy_zksync::provider::ZksyncProviderWithWallet;
use eyre::Result;

// Codegen from artifact.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20Mock,
    "lib/selic-contracts/out/ERC20Mock.sol/ERC20Mock.json"
);

pub async fn mint<P: Provider>(
    provider: &P,
    contract_address: Address,
    receiver: Address,
    amount: u64,
) -> Result<TransactionReceipt> {
    let erc20 = ERC20Mock::new(contract_address, provider);
    println!("ERC20 address: {}", erc20.address());
    let tx = erc20.mint(receiver, Uint::from(amount));
    let pending_tx = tx.send().await?;
    let receipt = pending_tx.get_receipt().await?;
    println!("ERC20 mint receipt: {:?}", receipt);
    Ok(receipt)
}

pub async fn mint_zk<P: ZksyncProviderWithWallet>(
    provider: &P,
    contract_address: Address,
    receiver: Address,
    amount: u64,
) -> Result<()> {
    let erc20 = ERC20Mock::new(contract_address, provider);
    println!("ERC20 address: {}", erc20.address());

    // let deposit_l1_receipt = provider
    //     .deposit(
    //         &DepositRequest::new(Uint::from(amount))
    //             .with_receiver(receiver)
    //             .with_token(contract_address),
    //         // use with_bridge_address to specify custom bridge address for the deposit
    //         //.with_bridge_address(address!("785185bbac3a09d447c679cf3420b206ea90be88")),
    //         // disable tokens auto approval if you plan to manage tokens allowance manually
    //         //.with_auto_approval(false),
    //         &l1_provider,
    //     )
    //     .await
    //     .unwrap();

    // let deposit_l2_receipt = deposit_l1_receipt
    //     .get_l2_tx()?
    //     .with_required_confirmations(1)
    //     .with_timeout(Some(std::time::Duration::from_secs(60 * 5)))
    //     .get_receipt()
    //     .await?;

    // println!("L2 deposit transaction receipt: {:#?}", deposit_l2_receipt);
    // Ok(receipt)
    Ok(())
}
