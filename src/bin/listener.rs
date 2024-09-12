use tech_challenge::api::config::Config;
use tech_challenge::rabbit::RabbitMQPagamentoUpdateSubscriber;
use std::sync::Arc;
use tokio::sync::Mutex;

use tech_challenge::external::postgres;
use tech_challenge::gateways::in_memory_pedido_gateway::InMemoryPedidoRepository;
use tech_challenge::gateways::in_memory_produto_gateway::InMemoryProdutoRepository;
use tech_challenge::gateways::{
    postgres_pedido_gateway::PostgresPedidoGateway,
    postgres_produto_gateway::PostgresProdutoRepository,
};

use tech_challenge::traits::{pedido_gateway::PedidoGateway, produto_gateway::ProdutoGateway};

#[tokio::main]
async fn main() {
    let config = Config::build();

    let (produto_gateway, pedido_gateway): (
        Arc<Mutex<dyn ProdutoGateway + Sync + Send>>,
        Arc<Mutex<dyn PedidoGateway + Sync + Send>>,
    ) = {
        if config.env == "test" {
            (
                Arc::new(Mutex::new(InMemoryProdutoRepository::new())),
                Arc::new(Mutex::new(InMemoryPedidoRepository::new())),
            )
        } else {
            print!("Connecting to database...");
            print!("Database URL: {}", config.db_url);
            let postgres_connection_manager =
                postgres::PgConnectionManager::new(config.db_url.clone())
                    .await
                    .unwrap();

            let postgres_client = Arc::new(postgres_connection_manager.client);

            let tables = postgres::get_tables();

            let produto_gateway = Arc::new(Mutex::new(
                PostgresProdutoRepository::new(postgres_client.clone(), tables.clone()).await,
            ));

            let pedido_gateway = Arc::new(Mutex::new(
                PostgresPedidoGateway::new(
                    postgres_client.clone(),
                    tables,
                    produto_gateway.clone(),
                )
                .await,
            ));

            (produto_gateway, pedido_gateway)
        }
    };

    let pagamento_update_subscriber = RabbitMQPagamentoUpdateSubscriber::new(
        config.clone(),
        produto_gateway.clone(),
        pedido_gateway.clone(),
    );

    match pagamento_update_subscriber
        .subscribe_pagamento_queue()
        .await
    {
        Ok(_) => {
            println!("Conectado ao RabbitMQ");
        }
        Err(e) => {
            println!("Erro ao conectar ao RabbitMQ: {:?}", e);
        }
    }
}
