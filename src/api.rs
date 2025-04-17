use crate::chain;
// use crate::mock;
use crate::provider::init_arbitrum_provider;
use crate::uniswap_v4::{
    uniswap_v4_subgraph,
    uniswap_v4_subgraph_entities::{
        BundleData, PoolData, PoolDayDataCollection, PoolManagerData, SwapData, TickData,
        TokenData, TokenDayDataCollection, TransactionData,
    },
};
use actix_web::{
    web::{self},
    HttpResponse, Responder,
};
// use alloy::core::primitives::Address;
// use alloy_zksync::{provider::zksync_provider, wallet::ZksyncWallet};

async fn chain_status() -> impl Responder {
    let provider = init_arbitrum_provider();
    #[derive(serde::Serialize)]
    struct ChainStatus {
        block_number: u64,
        chain_id: u64,
    }
    let mut chain_status = ChainStatus {
        block_number: 0,
        chain_id: 0,
    };
    let _root_provider = chain::chain_data::get_root_provider(&provider).await;
    match chain::chain_data::get_block_number(&provider).await {
        Ok(block_number) => chain_status.block_number = block_number,
        Err(e) => {
            println!("Failed to get block number: {}", e);
        }
    }
    match chain::chain_data::get_chain_id(&provider).await {
        Ok(chain_id) => chain_status.chain_id = chain_id,
        Err(e) => {
            println!("Failed to get chain ID: {}", e);
        }
    }
    if chain_status.block_number > 0 && chain_status.chain_id > 0 {
        let _block = chain::chain_data::get_block(&provider, chain_status.block_number).await;
        return HttpResponse::Ok().json(chain_status);
    } else {
        return HttpResponse::InternalServerError().json("Failed to get chain status");
    }
}

// async fn pool_manager_data() -> impl Responder {
//     let provider = init_arbitrum_provider();
//     let pool_manager_address = "0x360E68faCcca8cA495c1B759Fd9EEe466db9FB32"
//         .parse::<Address>()
//         .unwrap();

//     match uniswap_v4_pool_manager::get_owner(&provider, pool_manager_address).await {
//         Ok(owner) => HttpResponse::Ok().json(format!("Onwer: {:?}", owner)),
//         Err(e) => HttpResponse::InternalServerError().json(format!("Failed to get owner: {}", e)),
//     }
// }

async fn api_warning() -> impl Responder {
    HttpResponse::Ok().json("Warning: This is a test API. Use with caution.")
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("API está funcionando!")
}

async fn metrics_uniswap_v4() -> impl Responder {
    #[derive(serde::Serialize)]
    struct Metrics {
        pool_managers: PoolManagerData,
        tokens: TokenData,
        pools: PoolData,
        bundles: BundleData,
        swaps: SwapData,
        ticks: TickData,
        transactions: TransactionData,
        pool_day_data: PoolDayDataCollection,
        token_day_data: TokenDayDataCollection,
    }
    let mut metrics = Metrics {
        pool_managers: PoolManagerData::default(),
        tokens: TokenData::default(),
        pools: PoolData::default(),
        bundles: BundleData::default(),
        swaps: SwapData::default(),
        ticks: TickData::default(),
        transactions: TransactionData::default(),
        pool_day_data: PoolDayDataCollection::default(),
        token_day_data: TokenDayDataCollection::default(),
    };
    match uniswap_v4_subgraph::fetch_pool_managers().await {
        Ok(res) => metrics.pool_managers = res,
        Err(e) => {
            println!("Failed to fetch: {}", e);
        }
    }
    match uniswap_v4_subgraph::fetch_tokens().await {
        Ok(res) => metrics.tokens = res,
        Err(e) => {
            println!("Failed to fetch: {}", e);
        }
    }
    match uniswap_v4_subgraph::fetch_pools().await {
        Ok(res) => metrics.pools = res,
        Err(e) => {
            println!("Failed to fetch: {}", e);
        }
    }
    match uniswap_v4_subgraph::fetch_bundles().await {
        Ok(res) => metrics.bundles = res,
        Err(e) => {
            println!("Failed to fetch: {}", e);
        }
    }
    match uniswap_v4_subgraph::fetch_swaps().await {
        Ok(res) => metrics.swaps = res,
        Err(e) => {
            println!("Failed to fetch: {}", e);
        }
    }
    match uniswap_v4_subgraph::fetch_ticks().await {
        Ok(res) => metrics.ticks = res,
        Err(e) => {
            println!("Failed to fetch: {}", e);
        }
    }
    match uniswap_v4_subgraph::fetch_transactions().await {
        Ok(res) => metrics.transactions = res,
        Err(e) => {
            println!("Failed to fetch: {}", e);
        }
    }
    match uniswap_v4_subgraph::fetch_pool_day_data().await {
        Ok(res) => metrics.pool_day_data = res,
        Err(e) => {
            println!("Failed to fetch: {}", e);
        }
    }
    match uniswap_v4_subgraph::fetch_token_day_data().await {
        Ok(res) => metrics.token_day_data = res,
        Err(e) => {
            println!("Failed to fetch: {}", e);
        }
    }
    let response = serde_json::to_string(&metrics).unwrap();
    HttpResponse::Ok().body(response)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/api/health_check").to(health_check));
    cfg.service(
        web::scope("/api")
            .route("", web::get().to(api_warning))
            .route("/chain_status", web::get().to(chain_status))
            // .route("/pool_manager_data", web::get().to(pool_manager_data))
            .route("/metrics/uniswap-v4", web::get().to(metrics_uniswap_v4)),
    );
}
