pub mod tpft {
    pub mod get_tpft_data;
    pub mod tpft_create;
    pub mod tpft_deploy;
    pub mod tpft_mint;
}
pub mod utils {
    pub mod math;
    pub mod wallet;
}
pub mod pool {
    pub mod pool_manager_deploy;
    pub mod pool_manager_initialize;
}
pub mod mock {
    pub mod erc20_mock_deploy;
}
pub mod api;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(api::init))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
