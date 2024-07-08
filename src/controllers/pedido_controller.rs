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

    pub async fn atualiza_produto_by_categoria(
        &self,
        id: usize,
        categoria: &str,
        produto_id: usize,
    ) -> Result<Pedido, DomainError> {
        match categoria {
            "Lanche" => {
                self.pedidos_e_pagamentos_use_case
                    .adicionar_lanche_com_personalizacao(id, produto_id)
                    .await
            }
            "Acompanhamento" => {
                self.pedidos_e_pagamentos_use_case
                    .adicionar_acompanhamento(id, produto_id)
                    .await
            }
            "Bebida" => {
                self.pedidos_e_pagamentos_use_case
                    .adicionar_bebida(id, produto_id)
                    .await
            }
            _ => Err(DomainError::Invalid("Categoria inválida".to_string())),
        }
    }
}
