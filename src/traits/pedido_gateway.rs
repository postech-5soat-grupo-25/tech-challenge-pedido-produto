use mockall::*;

use crate::base::domain_error::DomainError;
use crate::entities::pedido::{Pedido, Status};
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

    async fn atualiza_status(
        &mut self,
        pedido_id: usize,
        status: Status,
    ) -> Result<Pedido, DomainError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_status_from_string() {
        let status = Status::from_str("Pago").unwrap();
        assert_eq!(status, Status::Pago);

        let status = Status::from_str("EmPreparacao").unwrap();
        assert_eq!(status, Status::EmPreparacao);

        let status = Status::from_str("Pronto").unwrap();
        assert_eq!(status, Status::Pronto);

        let status = Status::from_str("Pendente").unwrap();
        assert_eq!(status, Status::Pendente);

        let status = Status::from_str("Finalizado").unwrap();
        assert_eq!(status, Status::Finalizado);

        let status = Status::from_str("Cancelado").unwrap();
        assert_eq!(status, Status::Cancelado);

        let status = Status::from_str("Invalido").unwrap();
        assert_eq!(status, Status::Invalido);
    }

    #[tokio::test]
    async fn test_status_to_string() {
        let status = Status::Pago;
        assert_eq!(status.to_string(), "Pago");

        let status = Status::EmPreparacao;
        assert_eq!(status.to_string(), "EmPreparacao");

        let status = Status::Pronto;
        assert_eq!(status.to_string(), "Pronto");

        let status = Status::Pendente;
        assert_eq!(status.to_string(), "Pendente");

        let status = Status::Finalizado;
        assert_eq!(status.to_string(), "Finalizado");

        let status = Status::Cancelado;
        assert_eq!(status.to_string(), "Cancelado");

        let status = Status::Invalido;
        assert_eq!(status.to_string(), "Invalido");
    }
}