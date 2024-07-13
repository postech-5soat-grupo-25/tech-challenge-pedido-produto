use std::sync::Arc;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::{openapi, openapi_get_routes};
use tokio::sync::Mutex;

use crate::api::error_handling::ErrorResponse;
use crate::controllers::pedido_controller::PedidoController;
use crate::entities::pedido::Pedido;

use crate::traits::{
    pedido_gateway, produto_gateway,
};
use crate::use_cases::pedidos_e_pagamentos_use_case::CreatePedidoInput;

#[openapi(tag = "Pedidos")]
#[get("/")]
async fn get_pedidos(
    pedido_repository: &State<Arc<Mutex<dyn pedido_gateway::PedidoGateway + Sync + Send>>>,
    produto_repository: &State<Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>>>,
) -> Result<Json<Vec<Pedido>>, Status> {
    let pedido_controller = PedidoController::new(
        pedido_repository.inner().clone(),
        produto_repository.inner().clone(),
    );
    let pedidos = pedido_controller.get_pedidos().await?;
    Ok(Json(pedidos))
}

#[openapi(tag = "Pedidos")]
#[get("/<id>")]
async fn get_pedido_by_id(
    pedido_repository: &State<Arc<Mutex<dyn pedido_gateway::PedidoGateway + Sync + Send>>>,
    produto_repository: &State<Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>>>,
    id: usize,
) -> Result<Json<Pedido>, Status> {
    let pedido_controller = PedidoController::new(
        pedido_repository.inner().clone(),
        produto_repository.inner().clone(),
    );
    let pedido = pedido_controller.get_pedido_by_id(id).await?;
    Ok(Json(pedido))
}

#[openapi(tag = "Pedidos")]
#[post("/", data = "<pedido_input>")]
async fn post_novo_pedido(
    pedido_repository: &State<Arc<Mutex<dyn pedido_gateway::PedidoGateway + Sync + Send>>>,
    produto_repository: &State<Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>>>,
    pedido_input: Json<CreatePedidoInput>,
) -> Result<Json<Pedido>, Status> {
    let pedido_controller = PedidoController::new(
        pedido_repository.inner().clone(),
        produto_repository.inner().clone(),
    );
    let pedido_input = pedido_input.into_inner();
    let novo_pedido = pedido_controller.novo_pedido(pedido_input).await?;
    Ok(Json(novo_pedido))
}

#[openapi(tag = "Pedidos")]
#[get("/novos")]
async fn get_pedidos_novos(
    pedido_repository: &State<Arc<Mutex<dyn pedido_gateway::PedidoGateway + Sync + Send>>>,
    produto_repository: &State<Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>>>,
) -> Result<Json<Vec<Pedido>>, Status> {
    let pedido_controller = PedidoController::new(
        pedido_repository.inner().clone(),
        produto_repository.inner().clone(),
    );
    let pedidos_novos = pedido_controller.get_pedidos_novos().await?;
    Ok(Json(pedidos_novos))
}

#[openapi(tag = "Pedidos")]
#[put("/<id>/status/<status>")]
async fn put_status_pedido(
    pedido_repository: &State<Arc<Mutex<dyn pedido_gateway::PedidoGateway + Sync + Send>>>,
    produto_repository: &State<Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>>>,
    id: usize,
    status: &str,
) -> Result<Json<Pedido>, Status> {
    let pedido_controller = PedidoController::new(
        pedido_repository.inner().clone(),
        produto_repository.inner().clone(),
    );
    let pedido = pedido_controller.atualiza_status_pedido(id, status).await?;
    Ok(Json(pedido))
}


pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![
        get_pedidos,
        get_pedido_by_id,
        post_novo_pedido,
        get_pedidos_novos,
        put_status_pedido
    ]
}

#[catch(404)]
fn pedido_not_found() -> Json<ErrorResponse> {
    let error = ErrorResponse {
        msg: "Pedido não encontrado!".to_string(),
        status: 404,
    };
    Json(error)
}

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![pedido_not_found]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::{http::ContentType, local::blocking::Client};
    use crate::{base::domain_error::DomainError, entities::{ingredientes::Ingredientes, pedido, produto::{Categoria, Produto}}};

    fn create_valid_pedido() -> Pedido {
        let _now = "2021-08-01 00:00:00.000+0000".to_string();
        Pedido::new(
            1,
            None,
            None,
            None,
            None,
            None,
            pedido::Status::Pendente,
            _now.clone(),
            _now,
        )
    }

    #[test]
    fn test_get_pedidos() {
        let mut mock_pedido_gateway = pedido_gateway::MockPedidoGateway::new();
        mock_pedido_gateway
            .expect_lista_pedidos()
            .times(1)
            .returning(|| Ok(vec![]));

        let pedido_gateway: Arc<Mutex<dyn pedido_gateway::PedidoGateway + Sync + Send>> = Arc::new(Mutex::new(mock_pedido_gateway));
        let produto_gateway: Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>> = Arc::new(Mutex::new(produto_gateway::MockProdutoGateway::new()));

        let rocket = rocket::build()
            .mount("/", routes())
            .manage(pedido_gateway)
            .manage(produto_gateway);

        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_get_pedido_by_id() {
        let mut mock_pedido_gateway = pedido_gateway::MockPedidoGateway::new();
        mock_pedido_gateway
            .expect_get_pedido_by_id()
            .times(1)
            .returning(|_| Ok(create_valid_pedido()));

        let pedido_gateway: Arc<Mutex<dyn pedido_gateway::PedidoGateway + Sync + Send>> = Arc::new(Mutex::new(mock_pedido_gateway));
        let produto_gateway: Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>> = Arc::new(Mutex::new(produto_gateway::MockProdutoGateway::new()));

        let rocket = rocket::build()
            .mount("/", routes())
            .manage(pedido_gateway)
            .manage(produto_gateway);

        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/1").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_post_novo_pedido() {
        let mut mock_pedido_gateway = pedido_gateway::MockPedidoGateway::new();
        mock_pedido_gateway
            .expect_create_pedido()
            .times(1)
            .returning(|_| Ok(create_valid_pedido()));

        let mut mock_produto_gateway = produto_gateway::MockProdutoGateway::new();
        mock_produto_gateway.expect_get_produto_by_id().returning(|_| Ok(Produto::new(
            1,
            "Nome".to_string(),
            "Foto".to_string(),
            "Descricao".to_string(),
            Categoria::Lanche,
            1.0,
            Ingredientes::new(vec![
                "Pão".to_string(),
                "Hambúrguer".to_string(),
                "Queijo".to_string(),
            ]).unwrap(),
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        )));

        let pedido_gateway: Arc<Mutex<dyn pedido_gateway::PedidoGateway + Sync + Send>> = Arc::new(Mutex::new(mock_pedido_gateway));
        let produto_gateway: Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>> = Arc::new(Mutex::new(mock_produto_gateway));

        let rocket = rocket::build()
            .mount("/", routes())
            .manage(pedido_gateway)
            .manage(produto_gateway);

        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.post("/")
            .header(ContentType::JSON)
            .body(r##"{
                "lanche_id": 1
            }"##).dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_get_pedidos_novos() {
        let mut mock_pedido_gateway = pedido_gateway::MockPedidoGateway::new();
        mock_pedido_gateway
            .expect_get_pedidos_novos()
            .times(1)
            .returning(|| Ok(vec![create_valid_pedido()]));

        let pedido_gateway: Arc<Mutex<dyn pedido_gateway::PedidoGateway + Sync + Send>> = Arc::new(Mutex::new(mock_pedido_gateway));
        let produto_gateway: Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>> = Arc::new(Mutex::new(produto_gateway::MockProdutoGateway::new()));

        let rocket = rocket::build()
            .mount("/", routes())
            .manage(pedido_gateway)
            .manage(produto_gateway);

        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/novos").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_put_status_pedido() {
        let mut mock_pedido_gateway = pedido_gateway::MockPedidoGateway::new();
        mock_pedido_gateway
            .expect_atualiza_status()
            .times(1)
            .returning(|_, _| Ok(create_valid_pedido()));

        let pedido_gateway: Arc<Mutex<dyn pedido_gateway::PedidoGateway + Sync + Send>> = Arc::new(Mutex::new(mock_pedido_gateway));
        let produto_gateway: Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>> = Arc::new(Mutex::new(produto_gateway::MockProdutoGateway::new()));

        let rocket = rocket::build()
            .mount("/", routes())
            .manage(pedido_gateway)
            .manage(produto_gateway);

        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.put("/1/status/Pendente").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_handle_not_found() {
        let mut mock_pedido_gateway = pedido_gateway::MockPedidoGateway::new();
        mock_pedido_gateway
            .expect_get_pedido_by_id()
            .times(1)
            .returning(|_| Err(DomainError::NotFound));

        let pedido_gateway: Arc<Mutex<dyn pedido_gateway::PedidoGateway + Sync + Send>> = Arc::new(Mutex::new(mock_pedido_gateway));
        let produto_gateway: Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>> = Arc::new(Mutex::new(produto_gateway::MockProdutoGateway::new()));

        let rocket = rocket::build()
            .mount("/", routes())
            .register("/", catchers())
            .manage(pedido_gateway)
            .manage(produto_gateway);

        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/1").dispatch();

        assert_eq!(response.status(), Status::NotFound);

        let response = response.into_string().unwrap();
        assert_eq!(r##"{"msg":"Pedido não encontrado!","status":404}"##, response);
    }
}
