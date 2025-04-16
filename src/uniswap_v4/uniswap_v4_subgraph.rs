use crate::uniswap_v4::uniswap_v4_subgraph_entities::*;
use eyre::{eyre, Result};
use reqwest::Client;
use serde::de::DeserializeOwned;
// Você pode repetir esse padrão para outras entidades

#[derive(Debug, serde::Deserialize)]
struct GraphQLResponse<T> {
    data: T,
}

async fn fetch_from_subgraph<T: DeserializeOwned>(query: &str) -> Result<T> {
    let subgraph_url = "https://gateway.thegraph.com/api/9592b4886913eb17147d4c90176cc88f/subgraphs/id/DiYPVdygkfjDWhbxGSqAQxwBKmfKnkWQojqeM2rkLb3G";
    let client = Client::new();
    let response = client
        .post(subgraph_url)
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await
        .map_err(|e| eyre!("Erro ao enviar requisição: {}", e))?;

    let status = response.status();
    let text = response
        .text()
        .await
        .map_err(|e| eyre!("Erro ao ler resposta do subgraph: {}", e))?;

    if !status.is_success() {
        return Err(eyre!("Subgraph retornou erro HTTP ({}): {}", status, text));
    }

    match serde_json::from_str::<GraphQLResponse<T>>(&text) {
        Ok(parsed) => Ok(parsed.data),
        Err(e) => Err(eyre!(
            "Erro ao deserializar resposta do subgraph: {}\nResposta bruta: {}",
            e,
            text
        )),
    }
}

pub async fn fetch_pool_managers() -> Result<()> {
    let query = r#"
    {
        poolManagers(first: 5) {
            id
            poolCount
            txCount
            totalVolumeUSD
            totalVolumeETH
            totalFeesUSD
            totalFeesETH
            untrackedVolumeUSD
            totalValueLockedUSD
            totalValueLockedETH
            totalValueLockedUSDUntracked
            totalValueLockedETHUntracked
            owner
        }
    }"#;

    let data: PoolManagerData = fetch_from_subgraph(query).await?;
    for pm in data.pool_managers {
        println!(
            "PoolManager: {} | Total Volume USD: {}",
            pm.id, pm.total_volume_usd
        );
    }
    Ok(())
}

pub async fn fetch_tokens() -> Result<()> {
    let query = r#"
    {
        tokens(first: 5) {
            id
            symbol
            name
            decimals
            totalSupply
            volume
            volumeUSD
            untrackedVolumeUSD
            feesUSD
            txCount
            poolCount
            totalValueLocked
            totalValueLockedUSD
            totalValueLockedUSDUntracked
            derivedETH
        }
    }"#;

    let data: TokenData = fetch_from_subgraph(query).await?;
    for token in data.tokens {
        println!("Token: {} | Volume USD: {}", token.symbol, token.volume);
    }
    Ok(())
}

pub async fn fetch_pools() -> Result<()> {
    let query = r#"
    {
        pools(first: 5) {
            id
            createdAtTimestamp
            createdAtBlockNumber
            token0 { id symbol }
            token1 { id symbol }
            feeTier
            liquidity
            sqrtPrice
            token0Price
            token1Price
            tick
            tickSpacing
            observationIndex
            volumeToken0
            volumeToken1
            volumeUSD
            untrackedVolumeUSD
            feesUSD
            txCount
            collectedFeesToken0
            collectedFeesToken1
            collectedFeesUSD
            totalValueLockedToken0
            totalValueLockedToken1
            totalValueLockedETH
            totalValueLockedUSD
            totalValueLockedUSDUntracked
            liquidityProviderCount
            hooks
        }
    }"#;

    let data: PoolData = fetch_from_subgraph(query).await?;
    for pool in data.pools {
        println!(
            "Pool: {} | {}-{} | Liquidity: {}",
            pool.id, pool.token0.symbol, pool.token1.symbol, pool.liquidity
        );
    }
    Ok(())
}

pub async fn fetch_bundles() -> Result<()> {
    let query = r#"
    {
        bundles(first: 1) {
            id
            ethPriceUSD
        }
    }"#;

    let data: BundleData = fetch_from_subgraph(query).await?;
    for bundle in data.bundles {
        println!("ETH Price USD: {}", bundle.eth_price_usd);
    }
    Ok(())
}
