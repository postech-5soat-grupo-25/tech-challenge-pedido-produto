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
            self.delete_produto(produto.id().to_owned()).await.unwrap();
            self.create_produto(new_produto_data.clone()).await.unwrap();
            return Ok(new_produto_data.clone());
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


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initiates_produtos() {
        let produto_repository = InMemoryProdutoRepository::new();

        let produtos = produto_repository.get_produtos().await.unwrap();

        assert_eq!(produtos.len(), 1);

        let produto = produto_repository.get_produto_by_id(0).await.unwrap();

        assert_eq!(produto.id(), &0);
    }

    #[tokio::test]
    async fn test_adds_and_retrieves() {
        let mut produto_repository = InMemoryProdutoRepository::new();

        let categoria = Categoria::Lanche;

        let ingredientes = Ingredientes::new(vec![
            String::from("Carne"),
            String::from("Pao"),
            String::from("Alface"),
        ]).unwrap();

        let produto = Produto::new(
            1,
            "Hamburguer".to_string(),
            "hamburguer.png".to_string(),
            "hamburguer com uma carne e salada".to_string(),
            categoria,
            15.99,
            ingredientes,
            Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string(),
            Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string(),
        );

        let produto = produto_repository.create_produto(produto).await.unwrap();

        assert_eq!(produto.id(), &1);

        let produtos = produto_repository.get_produtos().await.unwrap();

        assert_eq!(produtos.len(), 2);

        let produto = produto_repository.get_produto_by_id(1).await.unwrap();

        assert_eq!(produto.id(), &1);
    }

    #[tokio::test]
    async fn test_update() {
        let mut produto_repository = InMemoryProdutoRepository::new();

        let mut produto = produto_repository.get_produto_by_id(0).await.unwrap();

        produto.set_categoria(Categoria::Bebida);

        let produto = produto_repository.update_produto(produto).await.unwrap();

        assert_eq!(produto.categoria(), &Categoria::Bebida);

        let produto = produto_repository.get_produto_by_id(0).await.unwrap();

        assert_eq!(produto.categoria(), &Categoria::Bebida);
    }

    #[tokio::test]
    async fn test_deletes() {
        let mut produto_repository = InMemoryProdutoRepository::new();

        let categoria = Categoria::Lanche;

        let ingredientes = Ingredientes::new(vec![
            String::from("Carne"),
            String::from("Pao"),
            String::from("Alface"),
        ]).unwrap();

        let produto = Produto::new(
            1,
            "Hamburguer".to_string(),
            "hamburguer.png".to_string(),
            "hamburguer com uma carne e salada".to_string(),
            categoria,
            15.99,
            ingredientes,
            Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string(),
            Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string(),
        );

        produto_repository.create_produto(produto).await.unwrap();

        produto_repository.delete_produto(0).await.unwrap();

        let produtos = produto_repository.get_produtos().await.unwrap();

        assert_eq!(produtos.len(), 1);
        assert_eq!(produtos[0].id(), &1);
    }

    #[tokio::test]
    async fn test_get_produtos_by_categoria() {
        let mut produto_repository = InMemoryProdutoRepository::new();

        let categoria = Categoria::Sobremesa;

        let ingredientes = Ingredientes::new(vec![
            String::from("Carne"),
            String::from("Pao"),
            String::from("Alface"),
        ]).unwrap();

        let produto = Produto::new(
            1,
            "Hamburguer".to_string(),
            "hamburguer.png".to_string(),
            "hamburguer com uma carne e salada".to_string(),
            categoria,
            15.99,
            ingredientes,
            Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string(),
            Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string(),
        );

        produto_repository.create_produto(produto).await.unwrap();

        let produtos = produto_repository.get_produtos_by_categoria(Categoria::Lanche).await.unwrap();

        assert_eq!(produtos.len(), 1);
    }

    #[tokio::test]
    async fn test_get_produtos_by_categoria_not_found() {
        let produto_repository = InMemoryProdutoRepository::new();

        let produtos = produto_repository.get_produtos_by_categoria(Categoria::Bebida).await.unwrap();

        assert_eq!(produtos.len(), 0);
    }
}