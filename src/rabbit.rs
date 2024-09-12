use crate::{
    api::config::Config,
    traits::{pedido_gateway::PedidoGateway, produto_gateway::ProdutoGateway},
    use_cases::pedidos_e_pagamentos_use_case::{InfoPagamenmto, PedidosEPagamentosUseCase},
};
use async_global_executor;
use futures_lite::stream::StreamExt;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties, Result};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio;

#[derive(Clone)]
pub struct RabbitMQPagamentoUpdateSubscriber {
    config: Config,
    pedido_e_pagamento_use_case: PedidosEPagamentosUseCase,
}

static MODULE_NAME: &str = "==RabbitMQ==";

impl RabbitMQPagamentoUpdateSubscriber {
    pub fn new(
        config: Config,
        produto_gateway: Arc<Mutex<dyn ProdutoGateway + Sync + Send>>,
        pedido_gateway: Arc<Mutex<dyn PedidoGateway + Sync + Send>>,
    ) -> Self {
        let pedido_e_pagamento_use_case =
            PedidosEPagamentosUseCase::new(pedido_gateway, produto_gateway);
        Self {
            config,
            pedido_e_pagamento_use_case,
        }
    }

    pub async fn subscribe_pagamento_queue(
        self,
    ) -> Result<()> {
        let res: Result<()> = async_global_executor::block_on(async {
            let conn = Connection::connect(self.config.rabbitmq_addr.as_str(), ConnectionProperties::default()).await?;
            let channel = conn.create_channel().await?;
            let mut consumer = channel
                .basic_consume(
                    self.config.queue_name.as_str(),
                    "pedido-pagamento-service",
                    BasicConsumeOptions::default(),
                    FieldTable::default(),
                )
                .await?;

            println!("{} Conectado ao RabbitMQ", MODULE_NAME);
            while let Some(delivery) = consumer.next().await {
                let delivery = delivery.expect("error in consumer");
                delivery.ack(BasicAckOptions::default()).await?;
                println!("{} Atualização de pagamento recebida", MODULE_NAME);
                let json_string = std::str::from_utf8(&delivery.data).unwrap();
                let update_input = serde_json::from_str::<InfoPagamenmto>(json_string);
                match update_input {
                    Ok(update_input) => {
                        println!("{} {:?}", MODULE_NAME, update_input);
                        match self.pedido_e_pagamento_use_case.atualiza_pagamento(update_input.clone()).await
                        {
                            Ok(pedido) => {
                                println!("{} Pedido atualizado: {:?} | Status do Pagamento: {:?}", MODULE_NAME, pedido.id(), pedido.status());
                            }
                            Err(e) => {
                                println!("{} Erro ao atualizar pedido: {:?}", MODULE_NAME, e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("{} Erro ao desserializar mensagem: {:?}", MODULE_NAME, e);
                    }
                }
            }
            Ok(())
        });
        res
    }
}
