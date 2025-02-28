use alloy::{hex, signers::local::PrivateKeySigner};
use eyre::Result;

#[tokio::main]
pub async fn create() -> Result<PrivateKeySigner> {
    let signer = PrivateKeySigner::random();
    let public_key = signer.address();
    let private_key = signer.credential();
    // Convert the private key to a hex string using alloy's hex module.
    let private_key_hex = format!("0x{}", hex::encode(private_key.to_bytes()));
    println!("Signer Public Key: {}", public_key);
    println!("Signer Private Key: {:?}", private_key_hex);
    // Deploy the `Counter` contract.

    Ok(signer)
}
