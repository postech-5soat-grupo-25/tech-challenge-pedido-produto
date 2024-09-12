use rocket::response::Redirect;
use rocket::{Build, Rocket};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::*;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::error_handling::generic_catchers;
use super::routes::{pedido_route, produto_route};
use crate::adapters::{
    api_key_validator::ApiKeyValidator, user_group_validator::UserGroupValidator,
};
use crate::api::config::Config;
use crate::external::postgres;
use crate::gateways::in_memory_pedido_gateway::InMemoryPedidoRepository;
use crate::gateways::in_memory_produto_gateway::InMemoryProdutoRepository;
use crate::gateways::{
    postgres_pedido_gateway::PostgresPedidoGateway,
    postgres_produto_gateway::PostgresProdutoRepository,
};
use crate::traits::api_key_validator_adapter::ApiKeyValidatorAdapter;
use crate::traits::user_group_validator_adapter::UserGroupValidatorAdapter;
use crate::traits::{pedido_gateway::PedidoGateway, produto_gateway::ProdutoGateway};

#[get("/")]
fn redirect_to_docs() -> Redirect {
    Redirect::to(uri!("/docs"))
}

pub async fn main() -> Rocket<Build> {
    let config = Config::build();

    if config.env == "test" {
        println!("Running test environment");
    }

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

    let server_config = rocket::Config::figment()
        .merge(("address", IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))))
        .merge(("port", 3000));

    let api_key_validator = ApiKeyValidator::new(config.api_key.clone());
    let api_key_validator: Arc<dyn ApiKeyValidatorAdapter + Sync + Send> =
        Arc::new(api_key_validator);

    let user_group_validator = UserGroupValidator::new();
    let user_group_validator: Arc<dyn UserGroupValidatorAdapter + Sync + Send> =
        Arc::new(user_group_validator);

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
        .manage(produto_gateway)
        .manage(pedido_gateway)
        .manage(api_key_validator)
        .manage(user_group_validator)
        .configure(server_config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::{Header, Status};
    use rocket::local::{asynchronous, blocking};
    use std::env;

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

        let client = asynchronous::Client::tracked(rocket)
            .await
            .expect("valid rocket instance");

        let response = client
            .get("/pedidos")
            .header(Header::new("UserGroup", "Admin"))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);
    }
}
