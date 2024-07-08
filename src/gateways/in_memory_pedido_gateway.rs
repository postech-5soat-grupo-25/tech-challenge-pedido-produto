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
            "mercadopago".to_string(),
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

async fn get_status_by_string(status: String) -> Status {
    let mut status_enum: Status = Status::Pendente;
    match status.as_str() {
        "pendente" => status_enum = Status::Pendente,
        "em_preparacao" => status_enum = Status::EmPreparacao,
        "pronto" => status_enum = Status::Pronto,
        "finalizado" => status_enum = Status::Finalizado,
        "set_pedido_cancelado" => status_enum = Status::Cancelado,
        &_ => status_enum = Status::Invalido,
    };
    return status_enum;
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



    async fn cadastrar_lanche(
        &mut self,
        pedido_id: usize,
        lanche: Produto,
    ) -> Result<Pedido, DomainError> {
        let pedidos = &mut self._pedidos;
        for pedido in pedidos.iter_mut() {
            if *pedido.id() == pedido_id {
                pedido.set_lanche(Some(lanche.clone()));
                return Ok(pedido.clone());
            }
        }
        Err(DomainError::NotFound)
    }

    async fn cadastrar_acompanhamento(
        &mut self,
        pedido_id: usize,
        acompanhamento: Produto,
    ) -> Result<Pedido, DomainError> {
        let pedidos = &mut self._pedidos;
        for pedido in pedidos.iter_mut() {
            if *pedido.id() == pedido_id {
                pedido.set_acompanhamento(Some(acompanhamento.clone()));
                return Ok(pedido.clone());
            }
        }
        Err(DomainError::NotFound)
    }

    async fn cadastrar_bebida(
        &mut self,
        pedido_id: usize,
        bebida: Produto,
    ) -> Result<Pedido, DomainError> {
        let pedidos = &mut self._pedidos;
        for pedido in pedidos.iter_mut() {
            if *pedido.id() == pedido_id {
                pedido.set_bebida(Some(bebida.clone()));
                return Ok(pedido.clone());
            }
        }
        Err(DomainError::NotFound)
    }
}

unsafe impl Sync for InMemoryPedidoRepository {}
unsafe impl Send for InMemoryPedidoRepository {}
