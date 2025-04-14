//! Example of deploying a contract from an artifact using the `sol!` macro to Anvil and interacting
//! with it.

use alloy::{
    eips::BlockId,
    providers::{Provider, RootProvider},
    rpc::types::Block,
};
use eyre::Result;

pub async fn get_block_number<P: Provider>(provider: &P) -> Result<u64> {
    // Deploy the `Counter` contract.
    let block_number = provider.get_block_number().await?;
    println!("Block number: {}", block_number);

    Ok(block_number)
}

pub async fn get_chain_id<P: Provider>(provider: &P) -> Result<u64> {
    // Deploy the `Counter` contract.
    let chain_id = provider.get_chain_id().await?;
    println!("Chain ID: {}", chain_id);

    Ok(chain_id)
}
pub async fn get_block<P: Provider>(provider: &P, block_number: BlockId) -> Result<Block> {
    // Deploy the `Counter` contract.
    let block = provider.get_block(block_number).await?;
    println!("Block: {:?}", block);

    Ok(block.unwrap())
}
pub async fn get_root_provider<P: Provider>(provider: &P) -> Result<&RootProvider> {
    // Deploy the `Counter` contract.
    let root_provider = provider.root();
    println!("Root Provider: {:?}", root_provider);

    Ok(root_provider)
}
