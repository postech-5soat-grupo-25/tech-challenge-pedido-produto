use rocket::response::Redirect;
use rocket::{Build, Rocket};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::*;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::error_handling::generic_catchers;
use super::routes::{pedido_route, produto_route};
use crate::api::config::{Config, Env};
use crate::external::postgres;
use crate::gateways::in_memory_pedido_gateway::InMemoryPedidoRepository;
use crate::gateways::in_memory_produto_gateway::InMemoryProdutoRepository;
use crate::gateways::{
    postgres_pedido_gateway::PostgresPedidoGateway,
    postgres_produto_gateway::PostgresProdutoRepository,
};
use crate::traits::{pedido_gateway::PedidoGateway, produto_gateway::ProdutoGateway};

#[get("/")]
fn redirect_to_docs() -> Redirect {
    Redirect::to(uri!("/docs"))
}

pub async fn main() -> Rocket<Build> {
    let config = Config::build();

    println!("Loading environment variables...");

    if config.env == Env::Test {
        println!("Running test environment");
    }

    let (produto_repository, pedido_repository): (
        Arc<Mutex<dyn ProdutoGateway + Sync + Send>>,
        Arc<Mutex<dyn PedidoGateway + Sync + Send>>,
    ) = {
        if config.env == Env::Test {
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

            let produto_repository = Arc::new(Mutex::new(
                PostgresProdutoRepository::new(postgres_client.clone(), tables.clone()).await,
            ));

            let pedido_repository = Arc::new(Mutex::new(
                PostgresPedidoGateway::new(
                    postgres_client.clone(),
                    tables,
                    produto_repository.clone(),
                )
                .await,
            ));

            (produto_repository, pedido_repository)
        }
    };

    let server_config = rocket::Config::figment()
        .merge(("address", IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))))
        .merge(("port", 3000));

    rocket::build()
        .mount("/", routes![redirect_to_docs])
        .register("/", generic_catchers())
        .mount(
            "/docs/",
            make_swagger_ui(&SwaggerUIConfig {
                urls: vec![
                    UrlObject::new("Produtos", "/produtos/openapi.json"),
                    UrlObject::new("Pedidos", "/pedidos/openapi.json"),
                ],
                ..Default::default()
            }),
        )
        .mount("/produtos", produto_route::routes())
        .mount("/pedidos", pedido_route::routes())
        .register("/produtos", produto_route::catchers())
        .register("/pedidos", pedido_route::catchers())
        .manage(produto_repository)
        .manage(pedido_repository)
        .configure(server_config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use rocket::local::{blocking, asynchronous};
    use rocket::http::Status;

    #[test]
    fn test_redirect_to_docs() {
        let rocket = rocket::build().mount("/", routes![redirect_to_docs]);
        let client = blocking::Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::SeeOther);
        assert_eq!(response.headers().get_one("Location"), Some("/docs"));
    }

    #[tokio::test]
    async fn test_main() {
        env::set_var("ENV", "test");

        let rocket = main().await;

        let client = asynchronous::Client::tracked(rocket).await.expect("valid rocket instance");

        let response = client.get("/pedidos").dispatch().await;

        assert_eq!(response.status(), Status::Ok);
    }
}
