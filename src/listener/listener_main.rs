use tokio_postgres::Client;
pub struct ListenerConfig {
    pub db_client: Client,
    pub url: String,
    pub private_key: String,
    pub contract_address: String,
    pub first_block: u64,
}
#[derive(Deserialize, Serialize)]
struct Contract {
    name: String,
    address: String,
}
use crate::listener::listener_erc20::listen_erc20;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::Read;

pub async fn start_listener(db_client: Client) {
    // usar a URL do ws:// do seu nó Ethereum
    let url = reqwest::Url::parse("ws://127.0.0.1:8546").expect("Invalid URL");
    // usar a private key do nó Ethereum
    let private_key = "8f2a55949038a9610f50fb23b5883af3b4ecb3c3bb792cbcefbd1542c692be63"; // Substitua pela sua chave privada                                                                                  // carregar contratos
    let mut file = File::open("./src/chain_data/contracts.json").expect("Failed to open file");

    // Ler o conteúdo do arquivo para uma string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    // Desserializar o conteúdo JSON em uma lista de contratos
    let contracts: Vec<Contract> = serde_json::from_str(&contents).expect("Failed to parse JSON");

    // Procurar o contrato ERC20
    let erc20_contract = contracts
        .iter()
        .find(|&contract| contract.name == "ERC20")
        .expect("ERC20 not found");

    let listener_erc20: ListenerConfig = ListenerConfig {
        url: url.to_string(),
        private_key: private_key.to_string(),
        contract_address: erc20_contract.address.to_string(),
        first_block: 0,
        db_client,
    };

    tokio::spawn(async {
        if let Err(e) = listen_erc20(listener_erc20).await {
            eprintln!("Error: {:?}", e);
        }
    });
}
