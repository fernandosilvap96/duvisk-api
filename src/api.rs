use std::str::FromStr;

use crate::mock;
//use crate::tpft;
use actix_web::{
    web::{self, Path},
    HttpResponse, Responder,
};
use alloy::{
    core::primitives::Address,
    network::EthereumWallet,
    providers::{ProviderBuilder, WalletProvider},
    signers::local::PrivateKeySigner,
};
// use alloy_zksync::{provider::zksync_provider, wallet::ZksyncWallet};
use reqwest;

async fn deploy_erc20_mock() -> impl Responder {
    let url = reqwest::Url::parse("http://127.0.0.1:8545").expect("Invalid URL");
    let private_key = "8f2a55949038a9610f50fb23b5883af3b4ecb3c3bb792cbcefbd1542c692be63"; // Substitua pela sua chave privada
    let signer = PrivateKeySigner::from_str(private_key).expect("Invalid private key");
    let wallet = EthereumWallet::from(signer.clone());
    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);
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
    let url = reqwest::Url::parse("http://127.0.0.1:8545").expect("Invalid URL");
    let private_key = "8f2a55949038a9610f50fb23b5883af3b4ecb3c3bb792cbcefbd1542c692be63"; // Substitua pela sua chave privada
    let signer = PrivateKeySigner::from_str(private_key).expect("Invalid private key");
    let wallet = EthereumWallet::from(signer.clone());
    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);
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
            .route("/test_routes", web::get().to(deploy_erc20_mock)),
    );
}
