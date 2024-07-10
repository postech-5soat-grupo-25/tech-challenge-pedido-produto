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
    pedido_gateway::PedidoGateway, produto_gateway::ProdutoGateway,
};
use crate::use_cases::pedidos_e_pagamentos_use_case::CreatePedidoInput;

#[openapi(tag = "Pedidos")]
#[get("/")]
async fn get_pedidos(
    pedido_repository: &State<Arc<Mutex<dyn PedidoGateway + Sync + Send>>>,
    produto_repository: &State<Arc<Mutex<dyn ProdutoGateway + Sync + Send>>>,
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
    pedido_repository: &State<Arc<Mutex<dyn PedidoGateway + Sync + Send>>>,
    produto_repository: &State<Arc<Mutex<dyn ProdutoGateway + Sync + Send>>>,
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
    pedido_repository: &State<Arc<Mutex<dyn PedidoGateway + Sync + Send>>>,
    produto_repository: &State<Arc<Mutex<dyn ProdutoGateway + Sync + Send>>>,
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
    pedido_repository: &State<Arc<Mutex<dyn PedidoGateway + Sync + Send>>>,
    produto_repository: &State<Arc<Mutex<dyn ProdutoGateway + Sync + Send>>>,
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
    pedido_repository: &State<Arc<Mutex<dyn PedidoGateway + Sync + Send>>>,
    produto_repository: &State<Arc<Mutex<dyn ProdutoGateway + Sync + Send>>>,
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
