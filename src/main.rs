pub mod utils {
    pub mod math;
    pub mod wallet;
}
pub mod mock {
    pub mod erc20_mock_deploy;
    pub mod erc20_mock_mint;
}
pub mod chain {
    pub mod chain_data;
}
pub mod uniswap_v4 {
    pub mod uniswap_v4_pool_manager;
    pub mod uniswap_v4_subgraph;
    pub mod uniswap_v4_subgraph_entities;
}

pub mod subgraph {
    pub mod subgraph;
    pub mod subgraph_entities;
}

pub mod api;
pub mod provider;

use actix_web::{App, HttpServer};
use reqwest::Client;
use tokio::task;
use tokio::time::{sleep, Duration};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let api_url = "127.0.0.1:7555".to_string();
    let server = HttpServer::new(|| App::new().configure(api::init))
        .bind(&api_url)?
        .run();

    // Dispara a API em uma task separada
    let server_handle = task::spawn(server);

    // Aguarda a API ficar disponível verificando o health check
    let client = Client::new();
    let health_check_url = "http://127.0.0.1:7555/api/health_check";

    // Aguarda até o health check retornar 200 OK
    loop {
        let response = client.get(health_check_url).send().await;
        if let Ok(resp) = response {
            if resp.status().is_success() {
                println!("API disponível no endereço: {}", api_url);
                break;
            }
        }
        println!("Aguardando a API... Tentando novamente...");
        sleep(Duration::from_secs(1)).await;
    }
    // Aguarda o servidor rodar indefinidamente
    server_handle.await??;

    Ok(())
}
