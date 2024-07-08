use mockall::*;

use crate::base::domain_error::DomainError;
use crate::entities::{
    pedido::{Pedido, Status},
    produto::Produto,
};
use std::fmt;
use std::str::FromStr;

impl FromStr for Status {
    type Err = ();

    fn from_str(input: &str) -> Result<Status, Self::Err> {
        match input {
            "Pago" => Ok(Status::Pago),
            "EmPreparacao" => Ok(Status::EmPreparacao),
            "Pronto" => Ok(Status::Pronto),
            "Pendente" => Ok(Status::Pendente),
            "Finalizado" => Ok(Status::Finalizado),
            "Cancelado" => Ok(Status::Cancelado),
            "Invalido" => Ok(Status::Invalido),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Status::Pago => "Pago",
                Status::EmPreparacao => "EmPreparacao",
                Status::Pronto => "Pronto",
                Status::Pendente => "Pendente",
                Status::Finalizado => "Finalizado",
                Status::Cancelado => "Cancelado",
                Status::Invalido => "Invalido",
            }
        )
    }
}

#[automock]
#[async_trait]
pub trait PedidoGateway {
    async fn create_pedido(&mut self, pedido: Pedido) -> Result<Pedido, DomainError>;

    async fn lista_pedidos(&mut self) -> Result<Vec<Pedido>, DomainError>;

    async fn get_pedidos_novos(&self) -> Result<Vec<Pedido>, DomainError>;

    async fn get_pedido_by_id(&self, pedido_id: usize) -> Result<Pedido, DomainError>;

    async fn cadastrar_lanche(
        &mut self,
        pedido_id: usize,
        lanche: Produto,
    ) -> Result<Pedido, DomainError>;

    async fn cadastrar_acompanhamento(
        &mut self,
        pedido_id: usize,
        acompanhamento: Produto,
    ) -> Result<Pedido, DomainError>;

    async fn cadastrar_bebida(
        &mut self,
        pedido_id: usize,
        bebida: Produto,
    ) -> Result<Pedido, DomainError>;

    async fn atualiza_status(
        &mut self,
        pedido_id: usize,
        status: Status,
    ) -> Result<Pedido, DomainError>;
}
