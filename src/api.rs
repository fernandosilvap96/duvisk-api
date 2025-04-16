use crate::chain;
use crate::mock;
use crate::provider::{init_arbitrum_provider, init_local_provider};
use crate::uniswap_v4;
use actix_web::{
    web::{self, Path},
    HttpResponse, Responder,
};
use alloy::core::primitives::Address;
use alloy::providers::WalletProvider;
use std::str::FromStr;

// use alloy_zksync::{provider::zksync_provider, wallet::ZksyncWallet};

async fn deploy_erc20_mock() -> impl Responder {
    let provider = init_local_provider();
    //println!("Provider: {:?}", provider);
    match mock::erc20_mock_deploy::deploy(&provider).await {
        Ok(erc20_address) => {
            HttpResponse::Ok().json(format!("ERC20 deployed at address: {}", erc20_address))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Failed to deploy ERC20: {}", e))
        }
    }
}

async fn mint_erc20_mock(contract_address: Path<String>) -> impl Responder {
    let provider = init_local_provider();
    let erc20_address = match Address::from_str(&contract_address.into_inner()) {
        Ok(addr) => addr,
        Err(_) => return HttpResponse::BadRequest().json("Invalid contract address format"),
    };
    //println!("Provider: {:?}", provider);
    match mock::erc20_mock_mint::mint(
        &provider,
        erc20_address,
        provider.default_signer_address(),
        1000,
    )
    .await
    {
        Ok(receipt) => HttpResponse::Ok().json(format!("Receipt: {:?}", receipt)),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to mint ERC20: {}", e)),
    }
}

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

async fn pool_manager_data() -> impl Responder {
    let provider = init_arbitrum_provider();
    let pool_manager_address = "0x360E68faCcca8cA495c1B759Fd9EEe466db9FB32"
        .parse::<Address>()
        .unwrap();

    match uniswap_v4::uniswap_v4_pool_manager::get_owner(&provider, pool_manager_address).await {
        Ok(owner) => HttpResponse::Ok().json(format!("Onwer: {:?}", owner)),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to get owner: {}", e)),
    }
}
// async fn mint_erc20_zk_mock() -> impl Responder {
//     let l2_url = reqwest::Url::parse("http://127.0.0.1:3050").expect("Invalid URL");
//     let private_key = "8f2a55949038a9610f50fb23b5883af3b4ecb3c3bb792cbcefbd1542c692be63"; // Substitua pela sua chave privada
//     let signer = PrivateKeySigner::from_str(private_key).expect("Invalid private key");
//     let zk_wallet: ZksyncWallet = ZksyncWallet::from(signer.clone());
//     let zk_provider = zksync_provider().wallet(zk_wallet).on_http(l2_url);
//     let erc20_address =
//         Address::from_str("0x42699A7612A82f1d9C36148af9C77354759b210b").expect("Invalid address");
//     //println!("Provider: {:?}", provider);
//     match mock::erc20_mock_mint::mint_zk(
//         &zk_provider,
//         erc20_address,
//         zk_provider.default_signer_address(),
//         1000,
//     )
//     .await
//     {
//         Ok(receipt) => HttpResponse::Ok().json(format!("Receipt: {:?}", receipt)),
//         Err(e) => HttpResponse::InternalServerError().json(format!("Failed to mint ERC20: {}", e)),
//     }
// }

async fn api_warning() -> impl Responder {
    HttpResponse::Ok().json("Warning: This is a test API. Use with caution.")
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("API está funcionando!")
}

async fn test_subgraph() -> impl Responder {
    match uniswap_v4::uniswap_v4_subgraph::fetch_pool_managers().await {
        Ok(res) => println!("Fetched {:?}: ", res),
        Err(e) => {
            println!("Failed to fetch: {}", e);
        }
    }
    match uniswap_v4::uniswap_v4_subgraph::fetch_tokens().await {
        Ok(res) => println!("Fetched {:?}: ", res),
        Err(e) => {
            println!("Failed to fetch: {}", e);
        }
    }
    match uniswap_v4::uniswap_v4_subgraph::fetch_pools().await {
        Ok(res) => println!("Fetched {:?}: ", res),
        Err(e) => {
            println!("Failed to fetch: {}", e);
        }
    }
    match uniswap_v4::uniswap_v4_subgraph::fetch_bundles().await {
        Ok(res) => println!("Fetched {:?}: ", res),
        Err(e) => {
            println!("Failed to fetch: {}", e);
        }
    }
    HttpResponse::Ok().body("Subgraph data fetched successfully!")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/api/health_check").to(health_check));
    cfg.service(
        web::scope("/api")
            .route("", web::get().to(api_warning))
            .route("/deploy_erc20_mock", web::get().to(deploy_erc20_mock))
            .route(
                "/mint_erc20_mock/{contract_address}",
                web::get().to(mint_erc20_mock),
            )
            .route("/test_routes", web::get().to(deploy_erc20_mock))
            .route("/chain_status", web::get().to(chain_status))
            .route("/pool_manager_data", web::get().to(pool_manager_data))
            .route("/test_subgraph", web::get().to(test_subgraph)),
    );
}
