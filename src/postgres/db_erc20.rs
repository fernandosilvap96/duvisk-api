use tokio_postgres::Client;

pub async fn insert_event(
    client: &Client,
    event_type: &str,
    from: &str,
    to: &str,
    value: u64,
    block_number: u64,
    tx_hash: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let query = "
        INSERT INTO erc20_events (event_type, from_address, to_address, value, block_number, transaction_hash)
        VALUES ($1, $2, $3, $4, $5, $6)";
    client
        .execute(
            query,
            &[
                &event_type,
                &from,
                &to,
                &value.to_string(),
                &block_number.to_string(),
                &tx_hash,
            ],
        )
        .await?;
    Ok(())
}
