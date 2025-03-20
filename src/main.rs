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
    pub mod erc20_mock_mint;
}
pub mod api;
pub mod postgres {
    pub mod db_erc20;
    pub mod db_main;
}
pub mod listener {
    pub mod listener_erc20;
    pub mod listener_main;
}

use actix_web::{App, HttpServer};
use listener::listener_main;
use postgres::db_main;
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
    // conectar o banco de dados
    let db_client = db_main::conectar()
        .await
        .expect("Erro ao conectar ao banco de dados");
    // iniciar o listener
    let _ = listener_main::start_listener(db_client).await;

    // Aguarda o servidor rodar indefinidamente
    server_handle.await??;

    Ok(())
}
