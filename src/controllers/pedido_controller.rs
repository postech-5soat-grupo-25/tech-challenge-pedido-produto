use std::sync::Arc;

use tokio::sync::Mutex;

use crate::base::domain_error::DomainError;
use crate::entities::pedido::{self, Pedido};
use crate::traits::{pedido_gateway::PedidoGateway, produto_gateway::ProdutoGateway};

use crate::use_cases::{
    pedidos_e_pagamentos_use_case::CreatePedidoInput,
    pedidos_e_pagamentos_use_case::PedidosEPagamentosUseCase,
    preparacao_e_entrega_use_case::PreparacaoeEntregaUseCase,
};

pub struct PedidoController {
    pedidos_e_pagamentos_use_case: PedidosEPagamentosUseCase,
    preparacao_e_entrega_use_case: PreparacaoeEntregaUseCase,
}

impl PedidoController {
    pub fn new(
        pedido_repository: Arc<Mutex<dyn PedidoGateway + Sync + Send>>,
        produto_repository: Arc<Mutex<dyn ProdutoGateway + Sync + Send>>,
    ) -> PedidoController {
        let pedidos_e_pagamentos_use_case = PedidosEPagamentosUseCase::new(
            pedido_repository.clone(),
            produto_repository,
        );
        let preparacao_e_entrega_use_case = PreparacaoeEntregaUseCase::new(pedido_repository);

        PedidoController {
            pedidos_e_pagamentos_use_case,
            preparacao_e_entrega_use_case,
        }
    }

    pub async fn get_pedidos(&self) -> Result<Vec<Pedido>, DomainError> {
        self.pedidos_e_pagamentos_use_case.lista_pedidos().await
    }

    pub async fn get_pedido_by_id(&self, id: usize) -> Result<Pedido, DomainError> {
        self.pedidos_e_pagamentos_use_case
            .seleciona_pedido_por_id(id)
            .await
    }

    pub async fn novo_pedido(
        &self,
        pedido_input: CreatePedidoInput,
    ) -> Result<Pedido, DomainError> {
        self.pedidos_e_pagamentos_use_case
            .novo_pedido(pedido_input)
            .await
    }

    pub async fn get_pedidos_novos(&self) -> Result<Vec<Pedido>, DomainError> {
        self.preparacao_e_entrega_use_case.get_pedidos_novos().await
    }

    pub async fn atualiza_status_pedido(
        &self,
        id: usize,
        status: &str,
    ) -> Result<Pedido, DomainError> {
        let status = match status {
            "Cancelado" => pedido::Status::Cancelado,
            "EmPreparacao" => pedido::Status::EmPreparacao,
            "Finalizado" => pedido::Status::Finalizado,
            "Invalido" => pedido::Status::Invalido,
            "Pago" => pedido::Status::Pago,
            "Pendente" => pedido::Status::Pendente,
            "Pronto" => pedido::Status::Pronto,
            _ => return Err(DomainError::Invalid("Status inválido".to_string())),
        };
        self.preparacao_e_entrega_use_case
            .atualiza_status(id, status)
            .await
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::cpf::Cpf;
    use crate::entities::ingredientes::Ingredientes;
    use crate::entities::pedido::Pedido;
    use crate::entities::produto::{Categoria, Produto};
    use crate::traits::pedido_gateway::MockPedidoGateway;
    use crate::traits::produto_gateway::MockProdutoGateway;
    use crate::use_cases::pedidos_e_pagamentos_use_case::CreatePedidoInput;
    use mockall::predicate::*;
    use pedido::Status;

    fn create_valid_produto(categoria: Categoria) -> Produto {
        let _now = "2021-08-01 00:00:00.000+0000".to_string();
        Produto::new(
            1,
            "Cheeseburger".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            categoria,
            9.99,
            Ingredientes::new(vec![
                "Pão".to_string(),
                "Hambúrguer".to_string(),
                "Queijo".to_string(),
            ])
            .unwrap(),
            _now.clone(),
            _now,
        )
    }

    fn create_valid_pedido() -> Pedido {
        let _now = "2021-08-01 00:00:00.000+0000".to_string();
        let cliente = Cpf::new("123.456.789-09".to_string()).unwrap();
        let produto = create_valid_produto(Categoria::Lanche);
        Pedido::new(
            1,
            Some(cliente),
            Some(produto),
            None,
            None,
            None,
            Status::Pendente,
            _now.clone(),
            _now,
        )
    }

    fn create_valid_input() -> CreatePedidoInput {
        CreatePedidoInput {
            cliente_id: None,
            lanche_id: None,
            acompanhamento_id: None,
            bebida_id: None,
        }
    }

    #[tokio::test]
    async fn test_get_pedidos() {
        let mut mock_pedido_gateway = MockPedidoGateway::new();
        mock_pedido_gateway
            .expect_lista_pedidos()
            .times(1)
            .returning(|| Ok(vec![]));

        let pedido_gateway = Arc::new(Mutex::new(mock_pedido_gateway));
        let produto_gateway = Arc::new(Mutex::new(MockProdutoGateway::new()));

        let controller = PedidoController::new(pedido_gateway, produto_gateway);

        let result = controller.get_pedidos().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_get_pedido_by_id() {
        let pedido_retornado = create_valid_pedido();
        let pedido_esperado = pedido_retornado.clone();

        let mut mock_pedido_gateway = MockPedidoGateway::new();
        mock_pedido_gateway
            .expect_get_pedido_by_id()
            .times(1)
            .with(eq(1))
            .returning(move |_| Ok(pedido_retornado.clone()));

        let pedido_gateway = Arc::new(Mutex::new(mock_pedido_gateway));
        let produto_gateway = Arc::new(Mutex::new(MockProdutoGateway::new()));

        let controller = PedidoController::new(pedido_gateway, produto_gateway);

        let result = controller.get_pedido_by_id(1).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id(), pedido_esperado.id());
    }

    #[tokio::test]
    async fn test_novo_pedido() {
        let mut mock_pedido_gateway = MockPedidoGateway::new();
        let pedido_retornado = create_valid_pedido();
        let pedido_esperado = create_valid_pedido();

        mock_pedido_gateway
            .expect_create_pedido()
            .times(1)
            .returning(move |_| Ok(pedido_retornado.clone()));

        let pedido_gateway = Arc::new(Mutex::new(mock_pedido_gateway));
        let produto_gateway = Arc::new(Mutex::new(MockProdutoGateway::new()));

        let controller = PedidoController::new(pedido_gateway, produto_gateway);

        let pedido_input = create_valid_input();

        let result = controller.novo_pedido(pedido_input).await;

        assert!(result.is_ok());

        assert_eq!(result.unwrap().id(), pedido_esperado.id());
    }

    #[tokio::test]
    async fn test_get_pedidos_novos() {
        let pedido_retornado = create_valid_pedido();

        let mut mock_pedido_gateway = MockPedidoGateway::new();
        mock_pedido_gateway
            .expect_get_pedidos_novos()
            .times(1)
            .returning(move || Ok(vec![pedido_retornado.clone()]));

        let pedido_gateway = Arc::new(Mutex::new(mock_pedido_gateway));
        let produto_gateway = Arc::new(Mutex::new(MockProdutoGateway::new()));

        let controller = PedidoController::new(pedido_gateway, produto_gateway);

        let result = controller.get_pedidos_novos().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_atualiza_status_pedido() {
        let pedido_retornado = create_valid_pedido();
        let pedido_esperado = create_valid_pedido();

        let mut mock_pedido_gateway = MockPedidoGateway::new();
        mock_pedido_gateway
            .expect_atualiza_status()
            .times(1)
            .with(eq(1), eq(Status::Finalizado))
            .returning(move |_, _| Ok(pedido_retornado.clone()));

        let pedido_gateway = Arc::new(Mutex::new(mock_pedido_gateway));
        let produto_gateway = Arc::new(Mutex::new(MockProdutoGateway::new()));

        let controller = PedidoController::new(pedido_gateway, produto_gateway);

        let result = controller.atualiza_status_pedido(1, "Finalizado").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id(), pedido_esperado.id());
    }
}
