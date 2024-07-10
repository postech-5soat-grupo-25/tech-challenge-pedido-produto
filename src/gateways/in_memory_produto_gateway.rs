use crate::{
    entities::produto::Produto,
    traits::produto_gateway::ProdutoGateway,
    base::domain_error::DomainError,
};

use chrono::Utc;
use crate::entities::produto::Categoria;
use crate::entities::ingredientes::Ingredientes;

use tokio::time::{sleep, Duration};

pub struct InMemoryProdutoRepository {
    _produto: Vec<Produto>,
}

impl InMemoryProdutoRepository {
    pub fn new() -> Self {
        let _id = 0;
        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();

        let categoria = Categoria::Lanche;

        let ingredientes = Ingredientes::new(vec![
            String::from("Carne"),
            String::from("Pao"),
            String::from("Alface"),
        ]).unwrap();

        let produto = Produto::new(
            _id,
            "Hamburguer".to_string(),
            "hamburguer.png".to_string(),
            "hamburguer com uma carne e salada".to_string(),
            categoria,
            15.99,
            ingredientes,
            _now.clone(),
            _now,
        );

        println!("Usando repositório em memória!");

        InMemoryProdutoRepository {
            _produto: vec![produto],
        }
    }
}

#[async_trait]
impl ProdutoGateway for InMemoryProdutoRepository {
    async fn get_produtos(&self) -> Result<Vec<Produto>, DomainError> {
        let produtos = self._produto.clone();
        sleep(Duration::from_secs(1)).await;
        Ok(produtos)
    }

    async fn get_produto_by_id(&self, id: usize) -> Result<Produto, DomainError> {
        sleep(Duration::from_secs(1)).await;
        for produto in &self._produto {
            if produto.id().to_owned() == id {
                return Ok(produto.clone());
            }
        }
        Err(DomainError::NotFound)
    }

    async fn get_produtos_by_categoria(&self, categoria: Categoria) -> Result<Vec<Produto>, DomainError> {
        sleep(Duration::from_secs(1)).await;
        let mut produtos = Vec::new();
        for produto in &self._produto {
            if produto.categoria().to_owned() == categoria {
                produtos.push(produto.clone());
            }
        }
        Ok(produtos)
    }

    async fn create_produto(&mut self, produto: Produto) -> Result<Produto, DomainError> {
        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();

        let actual_produto = Produto::new(
            self._produto.len(),
            produto.nome().to_string(),
            produto.foto().to_string(),
            produto.descricao().to_string(),
            produto.categoria().to_owned(),
            produto.preco().to_owned(),
            produto.ingredientes().to_owned(),
            _now.clone(),
            _now,
        );

        let mut produto_list = self._produto.clone();
        produto_list.push(actual_produto.clone());

        self._produto = produto_list;

        Ok(actual_produto)
    }

    async fn update_produto(&mut self, new_produto_data: Produto) -> Result<Produto, DomainError> {
        sleep(Duration::from_secs(1)).await;
        let mut produto_list = self._produto.clone();
        for produto in &mut produto_list.iter_mut() {
        if produto.id() == new_produto_data.id() {
            *produto = new_produto_data.clone();
            return Ok(produto.clone());
        }
        }
        Err(DomainError::NotFound)
    }

    async fn delete_produto(&mut self, id: usize) -> Result<(), DomainError> {
        let produto_list = &mut self._produto;
        for (index, produto) in produto_list.iter_mut().enumerate() {
            if produto.id().to_owned() == id {
                produto_list.remove(index);
                return Ok(());
            }
        }
        Err(DomainError::NotFound)
    }
}
