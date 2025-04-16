use serde::Deserialize;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct PoolManagerData {
    #[serde(rename = "poolManagers")]
    pub pool_managers: Vec<PoolManager>,
}

#[derive(Debug, Deserialize)]
pub struct Bundle {
    pub id: String,
    #[serde(rename = "ethPriceUSD")]
    pub eth_price_usd: String,
}

#[derive(Debug, Deserialize)]
pub struct BundleData {
    pub bundles: Vec<Bundle>,
}

#[derive(Debug, Deserialize)]
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
    #[serde(rename = "totalValueLockedUSDUntracked")]
    pub total_value_locked_usd_untracked: String,
    #[serde(rename = "derivedETH")]
    pub derived_eth: String,
}

#[derive(Debug, Deserialize)]
pub struct TokenData {
    pub tokens: Vec<Token>,
}

#[derive(Debug, Deserialize)]
pub struct TokenRef {
    pub id: String,
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct PoolData {
    pub pools: Vec<Pool>,
}
