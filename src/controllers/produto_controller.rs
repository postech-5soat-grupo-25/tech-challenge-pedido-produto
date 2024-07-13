use std::sync::Arc;
use tokio::sync::Mutex;

use crate::base::domain_error::DomainError;
use crate::entities::produto::Produto;
use crate::traits::produto_gateway::ProdutoGateway;
use crate::use_cases::gerenciamento_de_produtos_use_case::{CreateProdutoInput, ProdutoUseCase, UpdateProdutoInput};

pub struct ProdutoController {
    produto_use_case: ProdutoUseCase,
}

impl ProdutoController {
    pub fn new(produto_repository: Arc<Mutex<dyn ProdutoGateway + Sync + Send>>) -> ProdutoController {
        let produto_use_case = ProdutoUseCase::new(produto_repository);
        ProdutoController { produto_use_case }
    }

    pub async fn get_produto(
        &self,
    ) -> Result<Vec<Produto>, DomainError> {
        self.produto_use_case.get_produtos().await
    }

    pub async fn get_produto_by_id(
        &self,
        id: usize,
    ) -> Result<Produto, DomainError> {
        self.produto_use_case.get_produto_by_id(id).await
    }

    pub async fn create_produto(
        &self,
        produto_input: CreateProdutoInput,
    ) -> Result<Produto, DomainError> {
        self.produto_use_case.create_produto(produto_input).await
    }

    pub async fn update_produto(
        &self,
        id: usize,
        produto_input: UpdateProdutoInput,
    ) -> Result<Produto, DomainError> {
        self.produto_use_case.update_produto(id, produto_input).await
    }

    pub async fn delete_produto(
        &self,
        id: usize,
    ) -> Result<(), DomainError> {
        self.produto_use_case.delete_produto(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{entities::{ingredientes::Ingredientes, produto::Categoria}, traits::produto_gateway::MockProdutoGateway};

    #[tokio::test]
    async fn test_get_produto() {
        let mut mock_produto_gateway = MockProdutoGateway::new();
        mock_produto_gateway.expect_get_produtos().returning(|| Ok(vec![]));

        let produto_repository = Arc::new(Mutex::new(mock_produto_gateway));
        let produto_controller = ProdutoController::new(produto_repository);

        let result = produto_controller.get_produto().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_produto_by_id() {
        let mut mock_produto_gateway = MockProdutoGateway::new();
        mock_produto_gateway.expect_get_produto_by_id().returning(|_| Ok(Produto::new(
            1,
            "Nome".to_string(),
            "Foto".to_string(),
            "Descricao".to_string(),
            Categoria::Lanche,
            1.0,
            Ingredientes::new(vec![
                "Pão".to_string(),
                "Hambúrguer".to_string(),
                "Queijo".to_string(),
            ]).unwrap(),
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        )));

        let produto_repository = Arc::new(Mutex::new(mock_produto_gateway));
        let produto_controller = ProdutoController::new(produto_repository);

        let result = produto_controller.get_produto_by_id(1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_produto() {
        let mut mock_produto_gateway = MockProdutoGateway::new();
        mock_produto_gateway.expect_create_produto().returning(|_| Ok(Produto::new(
            1,
            "Nome".to_string(),
            "Foto".to_string(),
            "Descricao".to_string(),
            Categoria::Lanche,
            1.0,
            Ingredientes::new(vec![
                "Pão".to_string(),
                "Hambúrguer".to_string(),
                "Queijo".to_string(),
            ]).unwrap(),
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        )));

        let produto_repository = Arc::new(Mutex::new(mock_produto_gateway));
        let produto_controller = ProdutoController::new(produto_repository);

        let produto_input = CreateProdutoInput {
            nome: "Nome".to_string(),
            foto: "Foto".to_string(),
            descricao: "Descricao".to_string(),
            categoria: Categoria::Lanche,
            preco: 1.0,
            ingredientes: Ingredientes::new(vec![
                "Pão".to_string(),
                "Hambúrguer".to_string(),
                "Queijo".to_string(),
            ]).unwrap(),
        };

        let result = produto_controller.create_produto(produto_input).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_produto() {
        let mut mock_produto_gateway = MockProdutoGateway::new();
        mock_produto_gateway.expect_get_produto_by_id().returning(|_| Ok(Produto::new(
            1,
            "Nome".to_string(),
            "Foto".to_string(),
            "Descricao".to_string(),
            Categoria::Lanche,
            1.0,
            Ingredientes::new(vec![
                "Pão".to_string(),
                "Hambúrguer".to_string(),
                "Queijo".to_string(),
            ]).unwrap(),
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        )));
        mock_produto_gateway.expect_update_produto().returning(|produto| Ok(produto));

        let produto_repository = Arc::new(Mutex::new(mock_produto_gateway));
        let produto_controller = ProdutoController::new(produto_repository);

        let produto_input = UpdateProdutoInput {
            nome: Some("Nome".to_string()),
            foto: Some("Foto".to_string()),
            descricao: Some("Descricao".to_string()),
            categoria: Some(Categoria::Lanche),
            preco: Some(1.0),
            ingredientes: Some(Ingredientes::new(vec![
                "Pão".to_string(),
                "Hambúrguer".to_string(),
                "Queijo".to_string(),
            ]).unwrap()),
        };

        let result = produto_controller.update_produto(1, produto_input).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_produto() {
        let mut mock_produto_gateway = MockProdutoGateway::new();
        mock_produto_gateway.expect_delete_produto().returning(|_| Ok(()));

        let produto_repository = Arc::new(Mutex::new(mock_produto_gateway));
        let produto_controller = ProdutoController::new(produto_repository);

        let result = produto_controller.delete_produto(1).await;
        assert!(result.is_ok());
    }
}