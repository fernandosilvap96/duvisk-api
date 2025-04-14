use alloy::{
    network::EthereumWallet,
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, ProviderBuilder, RootProvider,
    },
    signers::local::PrivateKeySigner,
};
use std::str::FromStr;

pub type Provider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider,
>;

pub fn init_local_provider() -> Provider {
    let url = reqwest::Url::parse("http://127.0.0.1:8549").expect("Invalid URL");
    let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let signer = PrivateKeySigner::from_str(private_key).expect("Invalid private key");
    let wallet = EthereumWallet::from(signer);
    ProviderBuilder::new().wallet(wallet).on_http(url)
}

pub fn init_arbitrum_provider() -> Provider {
    // let alchemy_api_key = "cuXnOsap3ZEc_UUOfGOZwMPclnEabsdU";
    let alchemy_network_url =
        "https://arb-mainnet.g.alchemy.com/v2/cuXnOsap3ZEc_UUOfGOZwMPclnEabsdU";
    let url = reqwest::Url::parse(alchemy_network_url).expect("Invalid URL");
    let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let signer = PrivateKeySigner::from_str(private_key).expect("Invalid private key");
    let wallet = EthereumWallet::from(signer);
    ProviderBuilder::new().wallet(wallet).on_http(url)
}
