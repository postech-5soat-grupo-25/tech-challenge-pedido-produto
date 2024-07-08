use chrono::{DateTime, Utc};
use postgres_from_row::FromRow;
use std::collections::HashMap;

use crate::entities::produto::Categoria;
use crate::entities::produto::Produto;
use crate::entities::ingredientes::Ingredientes;

use super::table::{ColumnDefault, ColumnNullable, ColumnTypes};

pub fn get_produto_table_columns() -> HashMap<String, (ColumnTypes, ColumnNullable, ColumnDefault)>
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
        "nome".to_string(),
        (
            ColumnTypes::Text,
            ColumnNullable(false),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "foto".to_string(),
        (
            ColumnTypes::Text,
            ColumnNullable(false),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "descricao".to_string(),
        (
            ColumnTypes::Text,
            ColumnNullable(false),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "categoria".to_string(),
        (
            ColumnTypes::Text,
            ColumnNullable(false),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "preco".to_string(),
        (
            ColumnTypes::Float,
            ColumnNullable(false),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "ingredientes".to_string(),
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

impl FromRow for Produto {
    fn from_row(row: &tokio_postgres::Row) -> Self {
        let id: i32 = row.get("id");
        let preco: f64 = row.get("preco");

        let lista_ingredientes: Vec<String> = row.get("ingredientes");
        let ingredientes = match Ingredientes::new(lista_ingredientes) {
            Ok(ing) => ing,
            Err(e) => panic!("Failed to create Ingredientes: {:?}", e),
        };
        let data_criacao: std::time::SystemTime = row.get("data_criacao");
        let data_criacao: DateTime<Utc> = data_criacao.into();
        let data_atualizacao: std::time::SystemTime = row.get("data_atualizacao");
        let data_atualizacao: DateTime<Utc> = data_atualizacao.into();

        Produto::new(
            id as usize,
            row.get("nome"),
            row.get("foto"),
            row.get("descricao"),
            row.get::<_, &str>("categoria").parse::<Categoria>().unwrap(),
            preco,
            ingredientes,
            data_criacao.format("%Y-%m-%d %H:%M:%S%.3f%z").to_string(),
            data_atualizacao
                .format("%Y-%m-%d %H:%M:%S%.3f%z")
                .to_string(),
        )
    }

    fn try_from_row(row: &tokio_postgres::Row) -> Result<Self, tokio_postgres::Error> {
        let id: i32 = row.try_get("id")?;
        let preco: f64 = row.try_get("preco")?;

        let lista_ingredientes_json: tokio_postgres::types::Json<Vec<String>> =
            row.get("ingredientes");
        let lista_ingredientes = lista_ingredientes_json.0;
        let ingredientes = match Ingredientes::new(lista_ingredientes) {
            Ok(ing) => ing,
            Err(e) => panic!("Failed to create Ingredientes: {:?}", e),
        };

        Ok(Produto::new(
            id as usize,
            row.try_get("nome")?,
            row.try_get("foto")?,
            row.try_get("descricao")?,
            (row.try_get::<_, &str>("categoria")?).parse::<Categoria>().unwrap(),
            preco,
            ingredientes,
            row.get("data_criacao"),
            row.get("data_atualizacao"),
        ))
    }
}
