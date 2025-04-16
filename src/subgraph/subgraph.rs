use crate::subgraph::subgraph_entities::*;
use eyre::Result;
use reqwest::Client;
use serde::Deserialize;
// Você pode repetir esse padrão para outras entidades

#[derive(Debug, Deserialize)]
struct GraphQLResponse<T> {
    data: T,
}

async fn fetch_from_subgraph<T: for<'de> Deserialize<'de>>(query: &str) -> Result<T> {
    let subgraph_url = "https://api.studio.thegraph.com/query/109194/pool-manager/v0.0.1";
    let client = Client::new();
    let res = client
        .post(subgraph_url)
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await?
        .json::<GraphQLResponse<T>>()
        .await?;
    Ok(res.data)
}

pub async fn fetch_approvals() -> Result<()> {
    let data: ApprovalData = fetch_from_subgraph(FETCH_APPROVALS_QUERY).await?;
    for approval in data.approvals {
        println!("Approval ID: {} | Owner: {}", approval.id, approval.owner);
    }
    Ok(())
}

pub async fn fetch_swaps() -> Result<()> {
    let data: SwapData = fetch_from_subgraph(FETCH_SWAPS_QUERY).await?;
    for swap in data.swaps {
        println!("Swap ID: {} | Sender: {}", swap.id, swap.sender);
    }
    Ok(())
}

const FETCH_APPROVALS_QUERY: &str = r#"
{
    approvals(first: 5) {
        id
        owner
        spender
        internal_id
        amount
        blockNumber
        blockTimestamp
        transactionHash
    }
}
"#;

const FETCH_SWAPS_QUERY: &str = r#"
{
    swaps(first: 100) {
        id
        internal_id
        sender
        amount0
        amount1
        sqrtPriceX96
        liquidity
        tick
        fee
        blockNumber
        blockTimestamp
        transactionHash
    }
}
"#;
