use alloy::{
    core::primitives::Address,
    network::EthereumWallet,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::Filter,
    signers::local::PrivateKeySigner,
    sol,
    sol_types::SolEvent,
};
use eyre::Result;
use futures_util::stream::StreamExt;
use std::str::FromStr;
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20Mock,
    "lib/selic-contracts/out/ERC20Mock.sol/ERC20Mock.json"
);
use crate::db::db_erc20;
use crate::listener::listener_main::ListenerConfig;

pub async fn listen_erc20(config: ListenerConfig) -> Result<(), Box<dyn std::error::Error>> {
    let signer = PrivateKeySigner::from_str(&config.private_key).expect("Invalid private key");
    let wallet = EthereumWallet::from(signer.clone());
    let ws_connect = WsConnect::new(&config.url);
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .on_ws(ws_connect)
        .await?;
    let last_block = provider.get_block_number().await?;
    println!("Last block: {:?}", last_block);
    // Create a filter to get all logs from the contract.
    let erc20_address = Address::from_str(&config.contract_address).expect("Invalid address");
    let filter = Filter::new().address(erc20_address);
    let sub: alloy::pubsub::Subscription<alloy::rpc::types::Log> =
        provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    // carregar todos os registros do banco de dados.
    let events_saved = db_erc20::get_all_events(&config.db_client).await?;
    println!("Events saved: {:?}", events_saved);

    while let Some(log) = stream.next().await {
        match log.topic0() {
            // Match the `Approval(address,address,uint256)` event.
            Some(&ERC20Mock::Approval::SIGNATURE_HASH) => {
                let ERC20Mock::Approval {
                    owner,
                    spender,
                    value,
                } = log.log_decode()?.inner.data;
                println!("Approval from {owner} to {spender} of value {value}");
                let transaction_hash = log
                    .transaction_hash
                    .map_or_else(|| "None".to_string(), |hash| format!("{:#x}", hash));
                // procurar se a transaction_hash já existe no banco de dados
                if events_saved.iter().any(|x| x.5 == transaction_hash) {
                    continue;
                }
                // Inserir no banco de dados
                db_erc20::insert_event(
                    &config.db_client,
                    "Approval",
                    &owner.to_string(),
                    &spender.to_string(),
                    value
                        .try_into()
                        .unwrap_or_else(|_| panic!("Valor muito grande para u64")),
                    log.block_number.expect("Block number not found"),
                    &transaction_hash,
                )
                .await?;
            }
            // Match the `Transfer(address,address,uint256)` event.
            Some(&ERC20Mock::Transfer::SIGNATURE_HASH) => {
                let ERC20Mock::Transfer { from, to, value } = log.log_decode()?.inner.data;
                println!("Transfer from {from} to {to} of value {value}");
                let transaction_hash = log
                    .transaction_hash
                    .map_or_else(|| "None".to_string(), |hash| format!("{:#x}", hash));
                // procurar se a transaction_hash já existe no banco de dados
                if events_saved.iter().any(|x| x.5 == transaction_hash) {
                    continue;
                }
                // Inserir no banco de dados
                db_erc20::insert_event(
                    &config.db_client,
                    "Transfer",
                    &from.to_string(),
                    &to.to_string(),
                    value
                        .try_into()
                        .unwrap_or_else(|_| panic!("Valor muito grande para u64")),
                    log.block_number.expect("Block number not found"),
                    &transaction_hash,
                )
                .await?;
            }
            _ => (),
        }
    }
    Ok(())
}
