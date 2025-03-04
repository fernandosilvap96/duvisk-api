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
    let tpft_address = tpft::tpft_deploy::deploy(&provider, public_key).unwrap();
    let _ =
        tpft::tpft_create::create(&provider, tpft_address, acronym, code, maturity_date).unwrap();

    let _ = tpft::get_tpft_data::get(&provider, tpft_address, tpft_id);
    tpft::tpft_mint::mint(&provider, tpft_address, receiver, tpft_id, amount).unwrap();
    // criar pool manager
    let currency0 = mock::erc20_mock_deploy::deploy(&provider).unwrap();
    let currency1 = mock::erc20_mock_deploy::deploy(&provider).unwrap();
    let tick_spacing =
        utils::math::Int24::from_dec_str("60").expect("Value out of range for int24");
    let lp_fee = 3000;
    let starting_price = utils::math::Uint160::from(79228162514264337593543950336i128);
    let pool_manager_address = pool::pool_manager_deploy::deploy(
        &provider,
        currency0,
        currency1,
        lp_fee,
        tick_spacing,
        starting_price,
    )
    .unwrap();
    let _ = pool::pool_manager_initialize::initialize(&provider, pool_manager_address);
}
