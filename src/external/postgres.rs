
pub mod table;
pub mod pedido;
pub mod produto;

use tokio_postgres::{NoTls, Error, Client};
use tokio;

use crate::base::domain_error::DomainError;

use self::table::{Table, TablesNames};
use self::produto::get_produto_table_columns;
use self::pedido::get_pedido_table_columns;
pub struct PgConnectionManager {
  pub client: Client,
}

impl From<tokio_postgres::Error> for DomainError {
  fn from(e: tokio_postgres::Error) -> Self {
    eprintln!("Database error: {}", e);
    DomainError::Database(e.to_string())
  }
}

impl PgConnectionManager {
  pub async fn new(db_url: String) -> Result<Self, Error> {
    let (client, connection)  = tokio_postgres::connect(&db_url, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(PgConnectionManager { client })
  }
}

pub fn get_tables() -> Vec<Table> {
  vec![
    Table {
      name: TablesNames::Produto,
      columns: get_produto_table_columns(),
    },
    Table {
      name: TablesNames::Pedido,
      columns: get_pedido_table_columns(),
    },
  ]
}
