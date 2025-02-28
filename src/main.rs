pub mod counter {
    pub mod counter_deploy;
}
pub mod tpft {
    pub mod get_tpft_data;
    pub mod tpft_create;
    pub mod tpft_deploy;
    pub mod tpft_mint;
}
pub mod utils {
    pub mod wallet;
}
use alloy::providers::{ProviderBuilder, WalletProvider};

fn main() {
    let provider = ProviderBuilder::new().on_anvil_with_wallet();
    let public_key = provider.wallet().default_signer().address();
    // createTPFt
    let acronym = "LTN".to_string();
    let code = "1000".to_string();
    let maturity_date = 1000u64;
    let tpft_id = 1;
    //mintTPFt
    let receiver = public_key;
    let amount = 5000u64;
    //Starting call mods
    // create wallet
    //utils::wallet::create().unwrap();
    counter::counter_deploy::deploy(&provider).unwrap();
    let tpft_address = tpft::tpft_deploy::deploy(&provider, public_key).unwrap();
    let _ =
        tpft::tpft_create::create(&provider, tpft_address, acronym, code, maturity_date).unwrap();

    let _ = tpft::get_tpft_data::get(&provider, tpft_address, tpft_id);
    tpft::tpft_mint::mint(&provider, tpft_address, receiver, tpft_id, amount).unwrap();
}
