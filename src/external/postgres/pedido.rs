use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use postgres_from_row::FromRow;
use std::collections::HashMap;

use crate::entities::pedido::Status;

use super::table::{ColumnDefault, ColumnNullable, ColumnTypes};

pub fn get_pedido_table_columns() -> HashMap<String, (ColumnTypes, ColumnNullable, ColumnDefault)>
{
    let mut columns = HashMap::new();
    columns.insert(
        "id".to_string(),
        (
            ColumnTypes::Index,
            ColumnNullable(false),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "cliente".to_string(),
        (
            ColumnTypes::Char(11),
            ColumnNullable(true),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "lanche_id".to_string(),
        (
            ColumnTypes::Integer,
            ColumnNullable(true),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "acompanhamento_id".to_string(),
        (
            ColumnTypes::Integer,
            ColumnNullable(true),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "bebida_id".to_string(),
        (
            ColumnTypes::Integer,
            ColumnNullable(true),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "pagamento".to_string(),
        (
            ColumnTypes::Text,
            ColumnNullable(true),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "status".to_string(),
        (
            ColumnTypes::Text,
            ColumnNullable(false),
            ColumnDefault(None),
        ),
    );

    columns.insert(
        "data_criacao".to_string(),
        (
            ColumnTypes::Timestamp,
            ColumnNullable(true),
            ColumnDefault(Some("CURRENT_TIMESTAMP".to_string())),
        ),
    );
    columns.insert(
        "data_atualizacao".to_string(),
        (
            ColumnTypes::Timestamp,
            ColumnNullable(true),
            ColumnDefault(Some("CURRENT_TIMESTAMP".to_string())),
        ),
    );

    columns
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct ProxyPedido {
    id: usize,
    cliente: Option<String>,
    lanche_id: Option<usize>,
    acompanhamento_id: Option<usize>,
    bebida_id: Option<usize>,
    pagamento: String,
    status: Status,
    data_criacao: String,
    data_atualizacao: String,
}

impl ProxyPedido {
    pub fn new(
        id: usize,
        cliente: Option<String>,
        lanche_id: Option<usize>,
        acompanhamento_id: Option<usize>,
        bebida_id: Option<usize>,
        pagamento: String,
        status: Status,
        data_criacao: String,
        data_atualizacao: String,
    ) -> Self {
        ProxyPedido {
            id,
            cliente,
            lanche_id,
            acompanhamento_id,
            bebida_id,
            pagamento,
            status,
            data_criacao,
            data_atualizacao,
        }
    }

    pub fn id(&self) -> &usize {
        &self.id
    }

    pub fn cliente(&self) -> Option<&String> {
        self.cliente.as_ref()
    }

    pub fn lanche_id(&self) -> Option<&usize> {
        self.lanche_id.as_ref()
    }

    pub fn acompanhamento_id(&self) -> Option<&usize> {
        self.acompanhamento_id.as_ref()
    }

    pub fn bebida_id(&self) -> Option<&usize> {
        self.bebida_id.as_ref()
    }

    pub fn pagamento(&self) -> &String {
        &self.pagamento
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn data_criacao(&self) -> &String {
        &self.data_criacao
    }

    pub fn data_atualizacao(&self) -> &String {
        &self.data_atualizacao
    }
}

impl FromRow for ProxyPedido {
    fn from_row(row: &tokio_postgres::Row) -> Self {
        let id: i32 = row.get("id");
        let cliente: Option<String> = row.get("cliente");
        let lanche_id: Option<i32> = row.get("lanche_id");
        let acompanhamento_id: Option<i32> = row.get("acompanhamento_id");
        let bebida_id: Option<i32> = row.get("bebida_id");
        let data_criacao: std::time::SystemTime = row.get("data_criacao");
        let data_criacao: DateTime<Utc> = data_criacao.into();
        let data_atualizacao: std::time::SystemTime = row.get("data_atualizacao");
        let data_atualizacao: DateTime<Utc> = data_atualizacao.into();

    ProxyPedido::new(
            id as usize,
            cliente,
            lanche_id.map(|id| id as usize),
            acompanhamento_id.map(|id| id as usize),
            bebida_id.map(|id| id as usize),
            row.get("pagamento"),
            row.get::<_, &str>("status").parse::<Status>().unwrap(),
            data_criacao.format("%Y-%m-%d %H:%M:%S%.3f%z").to_string(),
            data_atualizacao.format("%Y-%m-%d %H:%M:%S%.3f%z").to_string(),
        )
    }

    fn try_from_row(row: &tokio_postgres::Row) -> Result<Self, tokio_postgres::Error> {
        let id: i32 = row.try_get("id")?;
        let cliente: Option<String> = row.try_get("cliente")?;
        let lanche_id: Option<i32> = row.try_get("lanche_id")?;
        let acompanhamento_id: Option<i32> = row.try_get("acompanhamento_id")?;
        let bebida_id: Option<i32> = row.try_get("bebida_id")?;

        let data_criacao: std::time::SystemTime = row.try_get("data_criacao")?;
        let data_criacao: DateTime<Utc> = data_criacao.into();
        let data_atualizacao: std::time::SystemTime = row.try_get("data_atualizacao")?;
        let data_atualizacao: DateTime<Utc> = data_atualizacao.into();

        Ok(ProxyPedido::new(
            id as usize,
            cliente,
            lanche_id.map(|id| id as usize),
            acompanhamento_id.map(|id| id as usize),
            bebida_id.map(|id| id as usize),
            row.try_get("pagamento")?,
            row.try_get::<_, &str>("status")?.parse::<Status>().unwrap(),
            data_criacao.format("%Y-%m-%d %H:%M:%S%.3f%z").to_string(),
            data_atualizacao.format("%Y-%m-%d %H:%M:%S%.3f%z").to_string(),
        ))
    }
}
