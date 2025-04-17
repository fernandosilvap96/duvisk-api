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

pub async fn fetch_pool_managers() -> Result<PoolManagerData> {
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
    // for pm in &data.pool_managers {
    //     println!(
    //         "PoolManager: {} | Total Volume USD: {}",
    //         pm.id, pm.total_volume_usd
    //     );
    // }
    Ok(data)
}

pub async fn fetch_ticks() -> Result<TickData> {
    let query = r#"
    {
        ticks(first: 5) {
            id
            poolAddress
            tickIdx
            liquidityGross
            liquidityNet
            price0
            price1
            createdAtTimestamp
            createdAtBlockNumber
        }
    }"#;

    let data: TickData = fetch_from_subgraph(query).await?;
    Ok(data)
}

pub async fn fetch_swaps() -> Result<SwapData> {
    let query = r#"
    {
        swaps(first: 5) {
            id
            timestamp
            amount0
            amount1
            amountUSD
            sqrtPriceX96
            tick
        }
    }"#;

    let data: SwapData = fetch_from_subgraph(query).await?;
    Ok(data)
}

pub async fn fetch_transactions() -> Result<TransactionData> {
    let query = r#"
    {
        transactions(first: 5) {
            id
            blockNumber
            timestamp
            gasUsed
            gasPrice
        }
    }"#;

    let data: TransactionData = fetch_from_subgraph(query).await?;
    Ok(data)
}

pub async fn fetch_pool_day_data() -> Result<PoolDayDataCollection> {
    let query = r#"
    {
        poolDayDatas(first: 5) {
            id
            date
            liquidity
            sqrtPrice
            token0Price
            token1Price
            tick
            tvlUSD
            volumeToken0
            volumeToken1
            volumeUSD
            feesUSD
            txCount
            open
            high
            low
            close
        }
    }"#;

    let data: PoolDayDataCollection = fetch_from_subgraph(query).await?;
    Ok(data)
}

pub async fn fetch_token_day_data() -> Result<TokenDayDataCollection> {
    let query = r#"
    {
        tokenDayDatas(first: 5) {
            id
            date
            volume
            volumeUSD
            untrackedVolumeUSD
            totalValueLocked
            totalValueLockedUSD
            priceUSD
            feesUSD
            open
            high
            low
            close
        }
    }"#;

    let data: TokenDayDataCollection = fetch_from_subgraph(query).await?;
    Ok(data)
}

pub async fn fetch_tokens() -> Result<TokenData> {
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
    // for token in &data.tokens {
    //     println!("Token: {} | Volume USD: {}", token.symbol, token.volume);
    // }
    Ok(data)
}

pub async fn fetch_pools() -> Result<PoolData> {
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
    // for pool in &data.pools {
    //     println!(
    //         "Pool: {} | {}-{} | Liquidity: {}",
    //         pool.id, pool.token0.symbol, pool.token1.symbol, pool.liquidity
    //     );
    // }
    Ok(data)
}

pub async fn fetch_bundles() -> Result<BundleData> {
    let query = r#"
    {
        bundles(first: 1) {
            id
            ethPriceUSD
        }
    }"#;

    let data: BundleData = fetch_from_subgraph(query).await?;
    // for bundle in &data.bundles {
    //     println!("ETH Price USD: {}", bundle.eth_price_usd);
    // }
    Ok(data)
}
