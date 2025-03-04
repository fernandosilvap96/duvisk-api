use crate::tpft;
use actix_web::{web, HttpResponse, Responder};
use alloy::providers::{ProviderBuilder, WalletProvider};

async fn deploy_tpft() -> impl Responder {
    let provider = ProviderBuilder::new().on_anvil_with_wallet();
    let public_key = provider.wallet().default_signer().address();
    match tpft::tpft_deploy::deploy(&provider, public_key).await {
        Ok(tpft_address) => {
            HttpResponse::Ok().json(format!("TPFt deployed at address: {}", tpft_address))
        }
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to deploy TPFt: {}", e)),
    }
}

async fn api_warning() -> impl Responder {
    HttpResponse::Ok().json("Warning: This is a test API. Use with caution.")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("", web::get().to(api_warning))
            .route("/deploy_tpft", web::get().to(deploy_tpft)),
    );
}
