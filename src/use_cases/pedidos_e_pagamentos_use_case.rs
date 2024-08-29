use crate::base::domain_error::DomainError;
use crate::entities::{
    pedido::{Pedido, Status},
    cpf::Cpf,
};
use crate::traits::{
    pedido_gateway::PedidoGateway, produto_gateway::ProdutoGateway,
};

use chrono::Utc;
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct CreatePedidoInput {
    pub cliente_id: Option<Cpf>,
    pub lanche_id: Option<usize>,
    pub acompanhamento_id: Option<usize>,
    pub bebida_id: Option<usize>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub enum StatusPagamento {
    Aprovado,
    Recusado
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct InfoPagamenmto {
    pub pedido_id: usize,
    pub pagamento_id: String,
    pub status: StatusPagamento,
}

#[derive(Clone)]
pub struct PedidosEPagamentosUseCase {
    pedido_repository: Arc<Mutex<dyn PedidoGateway + Sync + Send>>,
    produto_repository: Arc<Mutex<dyn ProdutoGateway + Sync + Send>>,
}

impl PedidosEPagamentosUseCase {
    pub fn new(
        pedido_repository: Arc<Mutex<dyn PedidoGateway + Sync + Send>>,
        produto_repository: Arc<Mutex<dyn ProdutoGateway + Sync + Send>>,
    ) -> Self {
        PedidosEPagamentosUseCase {
            pedido_repository,
            produto_repository,
        }
    }

    pub async fn lista_pedidos(&self) -> Result<Vec<Pedido>, DomainError> {
        let mut pedido_repository = self.pedido_repository.lock().await;
        pedido_repository.lista_pedidos().await
    }

    pub async fn seleciona_pedido_por_id(&self, id: usize) -> Result<Pedido, DomainError> {
        let pedido_repository = self.pedido_repository.lock().await;
        pedido_repository.get_pedido_by_id(id).await
    }

    pub async fn novo_pedido(
        &self,
        pedido_input: CreatePedidoInput,
    ) -> Result<Pedido, DomainError> {
        let lanche = if let Some(lanche_id) = pedido_input.lanche_id {
            let produto_repo = self.produto_repository.lock().await;
            Some(produto_repo.get_produto_by_id(lanche_id).await?)
        } else {
            None
        };

        let bebida = if let Some(bebida_id) = pedido_input.bebida_id {
            let produto_repo = self.produto_repository.lock().await;
            Some(produto_repo.get_produto_by_id(bebida_id).await?)
        } else {
            None
        };

        let acompanhamento = if let Some(acompanhamento_id) = pedido_input.acompanhamento_id {
            let produto_repo = self.produto_repository.lock().await;
            Some(produto_repo.get_produto_by_id(acompanhamento_id).await?)
        } else {
            None
        };

        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        let pedido = Pedido::new(
            0,
            pedido_input.cliente_id.clone(),
            lanche,
            acompanhamento,
            bebida,
            None,
            Status::Pendente,
            _now.clone(),
            _now.clone(),
        );

        let mut pedido_repository = self.pedido_repository.lock().await;

        pedido_repository.create_pedido(pedido).await
    }

    pub async fn atualiza_pagamento(&self,  info_pagamento : InfoPagamenmto) -> Result<Pedido, DomainError> {
        let pedido_repository = self.pedido_repository.try_lock();
        if pedido_repository.is_err() {
            return Err(DomainError::Invalid("Erro ao acessar o banco de dados".to_string()));
        }
        
        let status = match info_pagamento.status {
            StatusPagamento::Aprovado => Status::Pago,
            StatusPagamento::Recusado => Status::Cancelado
        };
        
        pedido_repository.unwrap().atualiza_pagamento_status(info_pagamento.pedido_id, info_pagamento.pagamento_id, status).await
    }
}

unsafe impl Send for PedidosEPagamentosUseCase {}
unsafe impl Sync for PedidosEPagamentosUseCase {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{
        pedido_gateway::MockPedidoGateway,
        produto_gateway::MockProdutoGateway,
    };
    use std::sync::Arc;
    use tokio;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_lista_pedidos() {
        let mut mock = MockPedidoGateway::new();

        let returned_pedido = Pedido::new(
            1,
            None,
            None,
            None,
            None,
            Some("id_pagamento".to_string()),
            Status::Pendente,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_pedido = returned_pedido.clone();

        mock.expect_lista_pedidos()
            .times(1)
            .returning(move || Ok(vec![returned_pedido.clone()]));

        let use_case = PedidosEPagamentosUseCase::new(
            Arc::new(Mutex::new(mock)),
            Arc::new(Mutex::new(MockProdutoGateway::new())),
        );
        let result = use_case.lista_pedidos().await;
        assert_eq!(result.unwrap()[0].id(), expected_pedido.id());
    }

    #[tokio::test]
    async fn test_seleciona_pedido_por_id() {
        let mut mock = MockPedidoGateway::new();

        let returned_pedido = Pedido::new(
            1,
            None,
            None,
            None,
            None,
            Some("id_pagamento".to_string()),
            Status::Pendente,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_pedido = returned_pedido.clone();

        mock.expect_get_pedido_by_id()
            .times(1)
            .returning(move |_| Ok(returned_pedido.clone()));

        let use_case = PedidosEPagamentosUseCase::new(
            Arc::new(Mutex::new(mock)),
            Arc::new(Mutex::new(MockProdutoGateway::new())),
        );
        let result = use_case.seleciona_pedido_por_id(1).await;
        assert_eq!(result.unwrap().id(), expected_pedido.id());
    }

    #[tokio::test]
    async fn test_novo_pedido() {
        let mut mock_pedido_gateway = MockPedidoGateway::new();

        let returned_pedido = Pedido::new(
            1,
            Some(Cpf::new("123.456.789-09".to_string()).unwrap()),
            None,
            None,
            None,
            Some("id_pagamento".to_string()),
            Status::Pendente,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_pedido = returned_pedido.clone();

        mock_pedido_gateway
            .expect_create_pedido()
            .times(1)
            .returning(move |_| Ok(returned_pedido.clone()));

        let use_case = PedidosEPagamentosUseCase::new(
            Arc::new(Mutex::new(mock_pedido_gateway)),
            Arc::new(Mutex::new(MockProdutoGateway::new())),
        );

        let result = use_case
            .novo_pedido(CreatePedidoInput {
                cliente_id: Cpf::new("123.456.789-09".to_string()).ok(),
                lanche_id: None,
                acompanhamento_id: None,
                bebida_id: None,
            })
            .await;
        assert_eq!(result.unwrap().id(), expected_pedido.id());
    }

    #[tokio::test]
    async fn test_atualiza_pagamento_pago() {
        let mut mock_pedido_gateway = MockPedidoGateway::new();

        let returned_pedido = Pedido::new(
            1,
            None,
            None,
            None,
            None,
            Some("id_pagamento".to_string()),
            Status::EmPreparacao,
            "2021-10-10".to_string(),
            "2021-10-10".to_string()
        );

        let expected_pedido = returned_pedido.clone();

        mock_pedido_gateway.expect_atualiza_pagamento_status()
            .times(1)
            .withf(|id, pagamento, status| {
                *id == 1 && *pagamento == "id_pagamento".to_string() && *status == Status::Pago
            })
            .returning(move |_, _, _| Ok(returned_pedido.clone()));

        let use_case = PedidosEPagamentosUseCase::new(
            Arc::new(Mutex::new(mock_pedido_gateway)),
            Arc::new(Mutex::new(MockProdutoGateway::new())),
        );

        let result = use_case
            .atualiza_pagamento(
                InfoPagamenmto {
                    pedido_id: 1,
                    pagamento_id: "id_pagamento".to_string(),
                    status: StatusPagamento::Aprovado,
                },
            )
            .await;
        assert_eq!(result.unwrap().id(), expected_pedido.id());
    }

    #[tokio::test]
    async fn test_atualiza_pagamento_recusado() {
        let mut mock_pedido_gateway = MockPedidoGateway::new();

        let returned_pedido = Pedido::new(
            1,
            None,
            None,
            None,
            None,
            Some("id_pagamento".to_string()),
            Status::EmPreparacao,
            "2021-10-10".to_string(),
            "2021-10-10".to_string()
        );

        let expected_pedido = returned_pedido.clone();

        mock_pedido_gateway.expect_atualiza_pagamento_status()
            .times(1)
            .withf(|id, pagamento, status| {
                *id == 1 && *pagamento == "id_pagamento".to_string() && *status == Status::Cancelado
            })
            .returning(move |_, _, _| Ok(returned_pedido.clone()));

        let use_case = PedidosEPagamentosUseCase::new(
            Arc::new(Mutex::new(mock_pedido_gateway)),
            Arc::new(Mutex::new(MockProdutoGateway::new())),
        );

        let result = use_case
            .atualiza_pagamento(
                InfoPagamenmto {
                    pedido_id: 1,
                    pagamento_id: "id_pagamento".to_string(),
                    status: StatusPagamento::Recusado,
                },
            )
            .await;
        assert_eq!(result.unwrap().id(), expected_pedido.id());
    }
}
