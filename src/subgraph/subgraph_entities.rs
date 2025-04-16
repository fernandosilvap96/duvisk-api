use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Approval {
    pub id: String,
    pub owner: String,
    pub spender: String,
    #[serde(rename = "internal_id")]
    pub internal_id: String,
    pub amount: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct Donate {
    pub id: String,
    #[serde(rename = "internal_id")]
    pub internal_id: String,
    pub sender: String,
    pub amount0: String,
    pub amount1: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct Initialize {
    pub id: String,
    #[serde(rename = "internal_id")]
    pub internal_id: String,
    #[serde(rename = "currency0")]
    pub currency0: String,
    #[serde(rename = "currency1")]
    pub currency1: String,
    pub fee: String,
    #[serde(rename = "tickSpacing")]
    pub tick_spacing: String,
    pub hooks: String,
    #[serde(rename = "sqrtPriceX96")]
    pub sqrt_price_x96: String,
    pub tick: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct ModifyLiquidity {
    pub id: String,
    #[serde(rename = "internal_id")]
    pub internal_id: String,
    pub sender: String,
    #[serde(rename = "tickLower")]
    pub tick_lower: String,
    #[serde(rename = "tickUpper")]
    pub tick_upper: String,
    #[serde(rename = "liquidityDelta")]
    pub liquidity_delta: String,
    pub salt: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct OperatorSet {
    pub id: String,
    pub owner: String,
    pub operator: String,
    pub approved: bool,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct OwnershipTransferred {
    pub id: String,
    pub user: String,
    #[serde(rename = "newOwner")]
    pub new_owner: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct ProtocolFeeControllerUpdated {
    pub id: String,
    #[serde(rename = "protocolFeeController")]
    pub protocol_fee_controller: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct ProtocolFeeUpdated {
    pub id: String,
    #[serde(rename = "internal_id")]
    pub internal_id: String,
    #[serde(rename = "protocolFee")]
    pub protocol_fee: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct Swap {
    pub id: String,
    #[serde(rename = "internal_id")]
    pub internal_id: String,
    pub sender: String,
    // BigInt como String
    pub amount0: String,
    pub amount1: String,
    #[serde(rename = "sqrtPriceX96")]
    pub sqrt_price_x96: String,
    pub liquidity: String,
    // Ints reais (Graph envia como números)
    pub tick: i32,
    pub fee: u32,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct Transfer {
    pub id: String,
    pub caller: String,
    pub from: String,
    pub to: String,
    #[serde(rename = "internal_id")]
    pub internal_id: String,
    pub amount: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct ApprovalData {
    pub approvals: Vec<Approval>,
}

#[derive(Debug, Deserialize)]
pub struct SwapData {
    pub swaps: Vec<Swap>,
}
