use tokio_postgres::{Client, Config, Error, NoTls};

pub type DBPool = Client;

pub async fn init_db(database_url: &str) -> Result <DBPool, Error> {
    let config: Config = database_url.parse()?;
    let (client, connection) = config.connect(NoTls).await?;
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error : {}", e);
        }
    });

    Ok(client)
    


}

/* postgres table sql code : 
CREATE TABLE custom_details (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE
);
*/