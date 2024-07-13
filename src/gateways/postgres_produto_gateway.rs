use bytes::BytesMut;
use postgres_from_row::FromRow;
use std::error::Error;
use std::sync::Arc;
use tokio_postgres::types::{FromSql, ToSql, Type};
use tokio_postgres::Client;

use crate::{
    base::domain_error::DomainError, entities::produto::Categoria, entities::produto::Produto,
    traits::produto_gateway::ProdutoGateway,
};

use crate::external::postgres::table::Table;

pub struct PostgresProdutoRepository {
    client: Arc<Client>,
    tables: Vec<Table>,
}

const CREATE_PRODUCT: &str = "INSERT INTO produto (nome, foto, descricao, categoria, preco, ingredientes, data_criacao, data_atualizacao) VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) RETURNING id, nome, foto, descricao, CAST(categoria AS VARCHAR) AS categoria, preco, ingredientes, data_criacao, data_atualizacao";
const QUERY_PRODUCT_BY_ID: &str = "SELECT id, nome, foto, descricao, CAST(categoria AS VARCHAR) AS categoria, preco, ingredientes, data_criacao, data_atualizacao FROM produto WHERE id = $1";
const QUERY_PRODUCTS: &str = "SELECT id, nome, foto, descricao, CAST(categoria AS VARCHAR) AS categoria, preco, ingredientes, data_criacao, data_atualizacao FROM produto";
const UPDATE_PRODUCT: &str = "UPDATE produto SET nome = $1, foto = $2, descricao = $3, categoria = $4, preco = $5, ingredientes = $6, data_atualizacao = CURRENT_TIMESTAMP WHERE id = $7 RETURNING id, nome, foto, descricao, CAST(categoria AS VARCHAR) AS categoria, preco, ingredientes, data_criacao, data_atualizacao";
const DELETE_PRODUCT: &str = "DELETE FROM produto WHERE id = $1 RETURNING id, nome, foto, descricao, CAST(categoria AS VARCHAR) AS categoria, preco, ingredientes, data_criacao, data_atualizacao";

impl<'a> FromSql<'a> for Categoria {
    fn from_sql(
        _ty: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let value = std::str::from_utf8(raw)?;

        match value {
            "Lanche" => Ok(Categoria::Lanche),
            "Bebida" => Ok(Categoria::Bebida),
            "Acompanhamento" => Ok(Categoria::Acompanhamento),
            "Sobremesa" => Ok(Categoria::Sobremesa),
            _ => Err("Invalid categoria value".into()),
        }
    }
    fn accepts(_ty: &tokio_postgres::types::Type) -> bool {
        true
    }
}

impl ToSql for Categoria {
    fn to_sql(
        &self,
        _ty: &Type,
        out: &mut BytesMut,
    ) -> Result<tokio_postgres::types::IsNull, Box<dyn std::error::Error + 'static + Send + Sync>>
    {
        match self {
            Categoria::Lanche => out.extend_from_slice(b"Lanche"),
            Categoria::Bebida => out.extend_from_slice(b"Bebida"),
            Categoria::Acompanhamento => out.extend_from_slice(b"Acompanhamento"),
            Categoria::Sobremesa => out.extend_from_slice(b"Sobremesa"),
        }
        Ok(tokio_postgres::types::IsNull::No)
    }

    fn accepts(_ty: &Type) -> bool {
        true
    }

    fn to_sql_checked(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<tokio_postgres::types::IsNull, Box<dyn std::error::Error + 'static + Send + Sync>>
    {
        self.to_sql(ty, out)
    }
}

impl PostgresProdutoRepository {
    pub async fn new(client: Arc<Client>, tables: Vec<Table>) -> Self {
        let repo = PostgresProdutoRepository { client, tables };
        repo.check_for_tables().await;
        repo
    }

    async fn check_for_tables(&self) {
        for table in self.tables.iter() {
            let query = table.get_create_if_not_exists_query();
            self.client.execute(query.as_str(), &[]).await.unwrap();
        }
    }
}

#[async_trait]
impl ProdutoGateway for PostgresProdutoRepository {
    async fn get_produtos(&self) -> Result<Vec<Produto>, DomainError> {
        let produtos = self.client.query(QUERY_PRODUCTS, &[]).await.unwrap();
        let mut produtos_vec = Vec::new();
        for produto in produtos {
            produtos_vec.push(Produto::from_row(&produto));
        }
        Ok(produtos_vec)
    }

    async fn get_produto_by_id(&self, id: usize) -> Result<Produto, DomainError> {
        let id = id as i32;
        let produto = self.client.query_one(QUERY_PRODUCT_BY_ID, &[&id]).await;
        match produto {
            Ok(produto) => Ok(Produto::from_row(&produto)),
            Err(_) => Err(DomainError::NotFound),
        }
    }

    async fn create_produto(&mut self, produto: Produto) -> Result<Produto, DomainError> {
        let ingredientes = produto.ingredientes();
        let ingredientes_vec: Vec<String> = ingredientes.to_vec_string();
        let new_produto = self
            .client
            .query(
                CREATE_PRODUCT,
                &[
                    &produto.nome(),
                    &produto.foto(),
                    &produto.descricao(),
                    &produto.categoria(),
                    &produto.preco(),
                    &ingredientes_vec,
                ],
            )
            .await
            .unwrap();
        let new_produto = new_produto.get(0);
        match new_produto {
            Some(produto) => Ok(Produto::from_row(produto)),
            None => Err(DomainError::Invalid("Produto".to_string())),
        }
    }

    async fn update_produto(&mut self, new_produto_data: Produto) -> Result<Produto, DomainError> {
        let id = new_produto_data.id().clone() as i32;
        let ingredientes = new_produto_data.ingredientes();
        let ingredientes_vec: Vec<String> = ingredientes.to_vec_string();

        let updated_produto = self
            .client
            .query(
                UPDATE_PRODUCT,
                &[
                    &new_produto_data.nome(),
                    &new_produto_data.foto(),
                    &new_produto_data.descricao(),
                    &new_produto_data.categoria(),
                    &new_produto_data.preco(),
                    &ingredientes_vec,
                    &id,
                ],
            )
            .await
            .unwrap();
        let updated_produto = updated_produto.get(0);
        match updated_produto {
            Some(produto) => Ok(Produto::from_row(produto)),
            None => Err(DomainError::NotFound),
        }
    }

    async fn delete_produto(&mut self, id: usize) -> Result<(), DomainError> {
        let id = id as i32;
        let deleted_produto = self.client.query_one(DELETE_PRODUCT, &[&id]).await;
        match deleted_produto {
            Ok(_) => Ok(()),
            _ => Err(DomainError::NotFound),
        }
    }
}
