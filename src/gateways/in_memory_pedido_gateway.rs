use chrono::Utc;
use tokio::time::{sleep, Duration};

use crate::base::domain_error::DomainError;
use crate::entities::pedido::{Pedido, Status};
use crate::entities::produto::{Categoria, Produto};

use crate::entities::cpf::Cpf;
use crate::entities::ingredientes::Ingredientes;

use crate::traits::pedido_gateway::PedidoGateway;

#[derive(Clone)]
pub struct InMemoryPedidoRepository {
    _pedidos: Vec<Pedido>,
}

impl InMemoryPedidoRepository {
    pub fn new() -> Self {
        let current_date = Utc::now().naive_utc().format("%Y-%m-%d").to_string();

        let lanche = Produto::new(
            1,
            "Cheeseburger".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            Categoria::Lanche,
            9.99,
            Ingredientes::new(vec![
                "Pão".to_string(),
                "Hambúrguer".to_string(),
                "Queijo".to_string(),
            ])
            .unwrap(),
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        let pedido = Pedido::new(
            1,
            Some(Cpf::new("000.000.000-00".to_string()).unwrap()),
            Some(lanche),
            None,
            None,
            None,
            Status::Pendente,
            current_date.clone(),
            current_date.clone(),
        );


        println!("Usando repositório em memória!");

        InMemoryPedidoRepository {
            _pedidos: vec![pedido],
        }
    }
}

#[async_trait]
impl PedidoGateway for InMemoryPedidoRepository {
    async fn lista_pedidos(&mut self) -> Result<Vec<Pedido>, DomainError> {
        Ok(self._pedidos.clone())
    }

    async fn get_pedidos_novos(&self) -> Result<Vec<Pedido>, DomainError> {
        let mut pedidos: Vec<Pedido> = Vec::new();
        for pedido in &self._pedidos {
            if *pedido.status() == Status::Pendente {
                pedidos.push(pedido.clone());
            }
        }
        sleep(Duration::from_secs(1)).await;
        Ok(pedidos)
    }

    async fn atualiza_status(&mut self, id: usize, status: Status) -> Result<Pedido, DomainError> {
        let pedidos = &mut self._pedidos;
        if status == Status::Invalido {
            return Err::<Pedido, _>(DomainError::Invalid("status".to_string()));
        }
        for pedido in pedidos.iter_mut() {
            if *pedido.id() == id {
                pedido.set_status(status.clone());
                return Ok(pedido.clone());
            }
        }
        Err(DomainError::NotFound)
    }

    async fn create_pedido(&mut self, pedido: Pedido) -> Result<Pedido, DomainError> {
        let pedidos = &mut self._pedidos;
        pedidos.push(pedido.clone());
        Ok(pedido)
    }

    async fn get_pedido_by_id(&self, pedido_id: usize) -> Result<Pedido, DomainError> {
        let pedidos = &self._pedidos;
        for pedido in pedidos.iter() {
            if *pedido.id() == pedido_id {
                return Ok(pedido.clone());
            }
        }
        Err(DomainError::NotFound)
    }
}

unsafe impl Sync for InMemoryPedidoRepository {}
unsafe impl Send for InMemoryPedidoRepository {}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initiates_pedidos() {
        let mut pedido_repository = InMemoryPedidoRepository::new();

        let pedidos = pedido_repository.lista_pedidos().await.unwrap();

        assert_eq!(pedidos.len(), 1);

        let pedido = pedido_repository.get_pedido_by_id(1).await.unwrap();

        assert_eq!(pedido.id(), &1);
    }

    #[tokio::test]
    async fn test_adds_and_retrieves() {
        let mut pedido_repository = InMemoryPedidoRepository::new();

        let lanche = Produto::new(
            2,
            "Cheeseburger".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            Categoria::Lanche,
            9.99,
            Ingredientes::new(vec![
                "Pão".to_string(),
                "Hambúrguer".to_string(),
                "Queijo".to_string(),
            ])
            .unwrap(),
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        let pedido = Pedido::new(
            2,
            Some(Cpf::new("000.000.000-00".to_string()).unwrap()),
            Some(lanche),
            None,
            None,
            Some("mercadopago".to_string()),
            Status::EmPreparacao,
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        pedido_repository.create_pedido(pedido.clone()).await.unwrap();

        let pedidos = pedido_repository.lista_pedidos().await.unwrap();

        assert_eq!(pedidos.len(), 2);

        let pedido = pedido_repository.get_pedido_by_id(2).await.unwrap();

        assert_eq!(pedido.id(), &2);
    }

    #[tokio::test]
    async fn test_get_pedidos_novos() {
        let mut pedido_repository = InMemoryPedidoRepository::new();

        let lanche = Produto::new(
            2,
            "Cheeseburger".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            Categoria::Lanche,
            9.99,
            Ingredientes::new(vec![
                "Pão".to_string(),
                "Hambúrguer".to_string(),
                "Queijo".to_string(),
            ])
            .unwrap(),
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        let pedido = Pedido::new(
            2,
            Some(Cpf::new("000.000.000-00".to_string()).unwrap()),
            Some(lanche),
            None,
            None,
            Some("mercadopago".to_string()),
            Status::EmPreparacao,
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        pedido_repository.create_pedido(pedido.clone()).await.unwrap();

        let pedidos_novos = pedido_repository.get_pedidos_novos().await.unwrap();

        assert_eq!(pedidos_novos.len(), 1);
    }

    #[tokio::test]
    async fn test_get_atualiza_status() {
        let mut pedido_repository = InMemoryPedidoRepository::new();

        let pedido = pedido_repository.get_pedido_by_id(1).await.unwrap();

        assert_eq!(pedido.status(), &Status::Pendente);

        let pedido = pedido_repository.atualiza_status(1, Status::EmPreparacao).await.unwrap();

        assert_eq!(pedido.status(), &Status::EmPreparacao);
    }
}
