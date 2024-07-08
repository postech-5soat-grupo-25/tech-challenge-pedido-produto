use std::sync::Arc;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::{openapi, openapi_get_routes};
use tokio::sync::Mutex;

use crate::api::error_handling::ErrorResponse;
use crate::controllers::produto_controller::ProdutoController;
use crate::traits::produto_gateway::ProdutoGateway;
use crate::use_cases::gerenciamento_de_produtos_use_case::CreateProdutoInput;
use crate::entities::produto::Produto;

#[openapi(tag = "Produtos")]
#[get("/")]
async fn get_produto(
    produto_repository: &State<Arc<Mutex<dyn ProdutoGateway + Sync + Send>>>,
) -> Result<Json<Vec<Produto>>, Status> {
    let produto_controller = ProdutoController::new(produto_repository.inner().clone());
    let produtos = produto_controller.get_produto().await?;
    Ok(Json(produtos))
}

#[openapi(tag = "Produtos")]
#[get("/<id>")]
async fn get_produto_by_id(
    produto_repository: &State<Arc<Mutex<dyn ProdutoGateway + Sync + Send>>>,
    id: usize,
) -> Result<Json<Produto>, Status> {
    let produto_controller = ProdutoController::new(produto_repository.inner().clone());
    let produto = produto_controller.get_produto_by_id(id).await?;
    Ok(Json(produto))
}

#[openapi(tag = "Produtos")]
#[post("/", data = "<produto_input>")]
async fn create_produto(
    produto_repository: &State<Arc<Mutex<dyn ProdutoGateway + Sync + Send>>>,
    produto_input: Json<CreateProdutoInput>,
) -> Result<Json<Produto>, Status> {
    let produto_controller = ProdutoController::new(produto_repository.inner().clone());
    let produto_input = produto_input.into_inner();
    let produto = produto_controller.create_produto(produto_input).await?;
    Ok(Json(produto))
}

#[openapi(tag = "Produtos")]
#[put("/<id>", data = "<produto_input>")]
async fn update_produto(
    produto_repository: &State<Arc<Mutex<dyn ProdutoGateway + Sync + Send>>>,
    produto_input: Json<CreateProdutoInput>,
    id: usize,
) -> Result<Json<Produto>, Status> {
    let produto_controller = ProdutoController::new(produto_repository.inner().clone());
    let produto_input = produto_input.into_inner();
    let produto = produto_controller.update_produto(id, produto_input).await?;
    Ok(Json(produto))
}

#[openapi(tag = "Produtos")]
#[delete("/<id>")]
async fn delete_produto(
    produto_repository: &State<Arc<Mutex<dyn ProdutoGateway + Sync + Send>>>,
    id: usize,
) -> Result<Json<String>, Status> {
    let produto_controller = ProdutoController::new(produto_repository.inner().clone());
    produto_controller.delete_produto(id).await?;
    Ok(Json("success".to_string()))
}

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![get_produto, get_produto_by_id, create_produto, update_produto, delete_produto]
}

#[catch(404)]
fn produto_not_found() -> Json<ErrorResponse> {
    let error = ErrorResponse {
        msg: "Produto não encontrado!".to_string(),
        status: 404,
    };
    Json(error)
}

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![produto_not_found]
}