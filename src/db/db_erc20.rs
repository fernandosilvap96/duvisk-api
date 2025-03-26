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

pub async fn get_all_events(
    db_client: &Client,
) -> Result<Vec<(String, String, String, String, String, String)>, Box<dyn std::error::Error>> {
    let query = "SELECT event_type, from_address, to_address, value, block_number, transaction_hash FROM erc20_events";
    let rows = db_client.query(query, &[]).await?;

    let events = rows
        .iter()
        .map(|row| {
            let event_type: String = row.get(0);
            let from_address: String = row.get(1);
            let to_address: String = row.get(2);
            let value: String = row.get(3);
            let block_number: String = row.get(4);
            let transaction_hash: String = row.get(5);

            (
                event_type,
                from_address,
                to_address,
                value,
                block_number,
                transaction_hash,
            )
        })
        .collect::<Vec<_>>();

    Ok(events)
}
