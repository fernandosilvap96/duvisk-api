use tokio_postgres::{Client, Error, NoTls};

pub async fn conectar() -> Result<Client, Error> {
    let conn_str = "host=localhost user=fernando password=silva dbname=rust_database";
    let (client, connection) = match tokio_postgres::connect(conn_str, NoTls).await {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Erro ao conectar: {}", e);
            return Err(e);
        }
    };

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Erro na conexão: {}", e);
        }
    });

    let row = match client.query_one("SELECT version()", &[]).await {
        Ok(row) => row,
        Err(e) => {
            return Err(e);
        }
    };

    let version: String = row.get(0);
    println!("Conectado ao PostgreSQL! Versão: {}", version);

    Ok(client)
}
