use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct PoolManager {
    pub id: String,
    #[serde(rename = "poolCount")]
    pub pool_count: String,
    #[serde(rename = "txCount")]
    pub tx_count: String,
    #[serde(rename = "totalVolumeUSD")]
    pub total_volume_usd: String,
    #[serde(rename = "totalVolumeETH")]
    pub total_volume_eth: String,
    #[serde(rename = "totalFeesUSD")]
    pub total_fees_usd: String,
    #[serde(rename = "totalFeesETH")]
    pub total_fees_eth: String,
    #[serde(rename = "untrackedVolumeUSD")]
    pub untracked_volume_usd: String,
    #[serde(rename = "totalValueLockedUSD")]
    pub total_value_locked_usd: String,
    #[serde(rename = "totalValueLockedETH")]
    pub total_value_locked_eth: String,
    #[serde(rename = "totalValueLockedUSDUntracked")]
    pub total_value_locked_usd_untracked: String,
    #[serde(rename = "totalValueLockedETHUntracked")]
    pub total_value_locked_eth_untracked: String,
    pub owner: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tick {
    pub id: String,
    pub pool_address: Option<String>,
    #[serde(rename = "tickIdx")]
    pub tick_idx: String,
    #[serde(rename = "liquidityGross")]
    pub liquidity_gross: String,
    #[serde(rename = "liquidityNet")]
    pub liquidity_net: String,
    pub price0: String,
    pub price1: String,
    #[serde(rename = "createdAtTimestamp")]
    pub created_at_timestamp: String,
    #[serde(rename = "createdAtBlockNumber")]
    pub created_at_block_number: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub id: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    pub timestamp: String,
    #[serde(rename = "gasUsed")]
    pub gas_used: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Swap {
    pub id: String,
    pub timestamp: String,
    pub amount0: String,
    pub amount1: String,
    #[serde(rename = "amountUSD")]
    pub amount_usd: String,
    #[serde(rename = "sqrtPriceX96")]
    pub sqrt_price_x96: String,
    pub tick: String,
    #[serde(rename = "logIndex")]
    pub log_index: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModifyLiquidity {
    pub id: String,
    pub timestamp: String,
    pub amount: String,
    pub amount0: String,
    pub amount1: String,
    #[serde(rename = "tickLower")]
    pub tick_lower: String,
    #[serde(rename = "tickUpper")]
    pub tick_upper: String,
    #[serde(rename = "logIndex")]
    pub log_index: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UniswapDayData {
    pub id: String,
    pub date: String,
    #[serde(rename = "volumeETH")]
    pub volume_eth: String,
    #[serde(rename = "volumeUSD")]
    pub volume_usd: String,
    #[serde(rename = "volumeUSDUntracked")]
    pub volume_usd_untracked: String,
    #[serde(rename = "feesUSD")]
    pub fees_usd: String,
    #[serde(rename = "txCount")]
    pub tx_count: String,
    #[serde(rename = "tvlUSD")]
    pub tvl_usd: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PoolDayData {
    pub id: String,
    pub date: i64,
    pub liquidity: String,
    #[serde(rename = "sqrtPrice")]
    pub sqrt_price: String,
    #[serde(rename = "token0Price")]
    pub token0_price: String,
    #[serde(rename = "token1Price")]
    pub token1_price: String,
    pub tick: Option<String>,
    #[serde(rename = "tvlUSD")]
    pub tvl_usd: String,
    #[serde(rename = "volumeToken0")]
    pub volume_token0: String,
    #[serde(rename = "volumeToken1")]
    pub volume_token1: String,
    #[serde(rename = "volumeUSD")]
    pub volume_usd: String,
    #[serde(rename = "feesUSD")]
    pub fees_usd: String,
    #[serde(rename = "txCount")]
    pub tx_count: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PoolHourData {
    pub id: String,
    #[serde(rename = "periodStartUnix")]
    pub period_start_unix: String,
    pub liquidity: String,
    pub sqrt_price: String,
    #[serde(rename = "token0Price")]
    pub token0_price: String,
    #[serde(rename = "token1Price")]
    pub token1_price: String,
    pub tick: Option<String>,
    #[serde(rename = "tvlUSD")]
    pub tvl_usd: String,
    #[serde(rename = "volumeToken0")]
    pub volume_token0: String,
    #[serde(rename = "volumeToken1")]
    pub volume_token1: String,
    #[serde(rename = "volumeUSD")]
    pub volume_usd: String,
    #[serde(rename = "feesUSD")]
    pub fees_usd: String,
    #[serde(rename = "txCount")]
    pub tx_count: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenDayData {
    pub id: String,
    pub date: i64,
    pub volume: String,
    #[serde(rename = "volumeUSD")]
    pub volume_usd: String,
    #[serde(rename = "untrackedVolumeUSD")]
    pub untracked_volume_usd: String,
    #[serde(rename = "totalValueLocked")]
    pub total_value_locked: String,
    #[serde(rename = "totalValueLockedUSD")]
    pub total_value_locked_usd: String,
    #[serde(rename = "priceUSD")]
    pub price_usd: String,
    #[serde(rename = "feesUSD")]
    pub fees_usd: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenHourData {
    pub id: String,
    #[serde(rename = "periodStartUnix")]
    pub period_start_unix: String,
    pub volume: String,
    #[serde(rename = "volumeUSD")]
    pub volume_usd: String,
    #[serde(rename = "untrackedVolumeUSD")]
    pub untracked_volume_usd: String,
    #[serde(rename = "totalValueLocked")]
    pub total_value_locked: String,
    #[serde(rename = "totalValueLockedUSD")]
    pub total_value_locked_usd: String,
    #[serde(rename = "priceUSD")]
    pub price_usd: String,
    #[serde(rename = "feesUSD")]
    pub fees_usd: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
    pub id: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    pub owner: String,
    pub origin: String,
    #[serde(rename = "createdAtTimestamp")]
    pub created_at_timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Subscribe {
    pub id: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    pub address: String,
    #[serde(rename = "logIndex")]
    pub log_index: String,
    pub timestamp: String,
    pub origin: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Unsubscribe {
    pub id: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    pub address: String,
    #[serde(rename = "logIndex")]
    pub log_index: String,
    pub timestamp: String,
    pub origin: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transfer {
    pub id: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    pub from: String,
    pub to: String,
    #[serde(rename = "logIndex")]
    pub log_index: String,
    pub timestamp: String,
    pub origin: String,
}

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct PoolManagerData {
    #[serde(rename = "poolManagers")]
    pub pool_managers: Vec<PoolManager>,
}

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct Bundle {
    pub id: String,
    #[serde(rename = "ethPriceUSD")]
    pub eth_price_usd: String,
}

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct BundleData {
    pub bundles: Vec<Bundle>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub decimals: String,
    #[serde(rename = "totalSupply")]
    pub total_supply: String,
    pub volume: String,
    #[serde(rename = "volumeUSD")]
    pub volume_usd: String,
    #[serde(rename = "untrackedVolumeUSD")]
    pub untracked_volume_usd: String,
    #[serde(rename = "feesUSD")]
    pub fees_usd: String,
    #[serde(rename = "txCount")]
    pub tx_count: String,
    #[serde(rename = "poolCount")]
    pub pool_count: String,
    #[serde(rename = "totalValueLocked")]
    pub total_value_locked: String,
    #[serde(rename = "totalValueLockedUSD")]
    pub total_value_locked_usd: String,
    #[serde(rename = "derivedETH")]
    pub derived_eth: String,
}

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct TokenData {
    pub tokens: Vec<Token>,
}

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct TokenRef {
    pub id: String,
    pub symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pool {
    pub id: String,
    #[serde(rename = "createdAtTimestamp")]
    pub created_at_timestamp: String,
    #[serde(rename = "createdAtBlockNumber")]
    pub created_at_block_number: String,
    pub token0: TokenRef,
    pub token1: TokenRef,
    #[serde(rename = "feeTier")]
    pub fee_tier: String,
    pub liquidity: String,
    #[serde(rename = "sqrtPrice")]
    pub sqrt_price: String,
    #[serde(rename = "token0Price")]
    pub token0_price: String,
    #[serde(rename = "token1Price")]
    pub token1_price: String,
    pub tick: Option<String>,
    #[serde(rename = "tickSpacing")]
    pub tick_spacing: String,
    #[serde(rename = "observationIndex")]
    pub observation_index: String,
    #[serde(rename = "volumeToken0")]
    pub volume_token0: String,
    #[serde(rename = "volumeToken1")]
    pub volume_token1: String,
    #[serde(rename = "volumeUSD")]
    pub volume_usd: String,
    #[serde(rename = "untrackedVolumeUSD")]
    pub untracked_volume_usd: String,
    #[serde(rename = "feesUSD")]
    pub fees_usd: String,
    #[serde(rename = "txCount")]
    pub tx_count: String,
    #[serde(rename = "collectedFeesToken0")]
    pub collected_fees_token0: String,
    #[serde(rename = "collectedFeesToken1")]
    pub collected_fees_token1: String,
    #[serde(rename = "collectedFeesUSD")]
    pub collected_fees_usd: String,
    #[serde(rename = "totalValueLockedToken0")]
    pub total_value_locked_token0: String,
    #[serde(rename = "totalValueLockedToken1")]
    pub total_value_locked_token1: String,
    #[serde(rename = "totalValueLockedETH")]
    pub total_value_locked_eth: String,
    #[serde(rename = "totalValueLockedUSD")]
    pub total_value_locked_usd: String,
    #[serde(rename = "totalValueLockedUSDUntracked")]
    pub total_value_locked_usd_untracked: String,
    #[serde(rename = "liquidityProviderCount")]
    pub liquidity_provider_count: String,
    pub hooks: String,
}

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct PoolData {
    pub pools: Vec<Pool>,
}

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct TickData {
    pub ticks: Vec<Tick>,
}

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct SwapData {
    pub swaps: Vec<Swap>,
}

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct TransactionData {
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct PoolDayDataCollection {
    #[serde(rename = "poolDayDatas")]
    pub pool_day_datas: Vec<PoolDayData>,
}

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct TokenDayDataCollection {
    #[serde(rename = "tokenDayDatas")]
    pub token_day_datas: Vec<TokenDayData>,
}
