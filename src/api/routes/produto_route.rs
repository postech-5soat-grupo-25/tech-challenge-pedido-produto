use std::sync::Arc;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::{openapi, openapi_get_routes};
use tokio::sync::Mutex;

use crate::api::error_handling::ErrorResponse;
use crate::controllers::produto_controller::ProdutoController;
use crate::traits::produto_gateway;
use crate::use_cases::gerenciamento_de_produtos_use_case::{CreateProdutoInput, UpdateProdutoInput};
use crate::entities::produto::Produto;

#[openapi(tag = "Produtos")]
#[get("/")]
async fn get_produtos(
    produto_repository: &State<Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>>>,
) -> Result<Json<Vec<Produto>>, Status> {
    let produto_controller = ProdutoController::new(produto_repository.inner().clone());
    let produtos = produto_controller.get_produto().await?;
    Ok(Json(produtos))
}

#[openapi(tag = "Produtos")]
#[get("/<id>")]
async fn get_produto_by_id(
    produto_repository: &State<Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>>>,
    id: usize,
) -> Result<Json<Produto>, Status> {
    let produto_controller = ProdutoController::new(produto_repository.inner().clone());
    let produto = produto_controller.get_produto_by_id(id).await?;
    Ok(Json(produto))
}

#[openapi(tag = "Produtos")]
#[post("/", data = "<produto_input>")]
async fn create_produto(
    produto_repository: &State<Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>>>,
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
    produto_repository: &State<Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>>>,
    produto_input: Json<UpdateProdutoInput>,
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
    produto_repository: &State<Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>>>,
    id: usize,
) -> Result<Json<String>, Status> {
    let produto_controller = ProdutoController::new(produto_repository.inner().clone());
    produto_controller.delete_produto(id).await?;
    Ok(Json("success".to_string()))
}

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![get_produtos, get_produto_by_id, create_produto, update_produto, delete_produto]
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

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::{http::ContentType, local::blocking::Client};
    use crate::{base::domain_error::DomainError, entities::{ingredientes::Ingredientes, produto::{Categoria, Produto}}};

    fn create_valid_produto() -> Produto {
        Produto::new(
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
        )
    }

    #[test]
    fn test_get_produtos() {
        let mut mock_produto_gateway = produto_gateway::MockProdutoGateway::new();
        mock_produto_gateway
            .expect_get_produtos()
            .times(1)
            .returning(|| Ok(vec![create_valid_produto()]));

        let produto_gateway: Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>> = Arc::new(Mutex::new(mock_produto_gateway));

        let rocket = rocket::build()
            .mount("/", routes())
            .manage(produto_gateway);

        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_get_produto_by_id() {
        let mut mock_produto_gateway = produto_gateway::MockProdutoGateway::new();
        mock_produto_gateway
            .expect_get_produto_by_id()
            .times(1)
            .returning(|_| Ok(create_valid_produto()));

        let produto_gateway: Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>> = Arc::new(Mutex::new(mock_produto_gateway));

        let rocket = rocket::build()
            .mount("/", routes())
            .manage(produto_gateway);

        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/1").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_post_create_produto() {
        let mut mock_produto_gateway = produto_gateway::MockProdutoGateway::new();
        mock_produto_gateway
            .expect_create_produto()
            .times(1)
            .returning(|produto| Ok(produto));

        let produto_gateway: Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>> = Arc::new(Mutex::new(mock_produto_gateway));

        let rocket = rocket::build()
            .mount("/", routes())
            .manage(produto_gateway);

        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.post("/")
            .header(ContentType::JSON)
            .body(r##"{
                "nome": "teste",
                "foto": "foto.png",
                "descricao": "Descrição",
                "categoria": "Lanche",
                "preco": 9.50,
                "ingredientes": [
                    "Pão",
                    "Hambúrguer"
                ]
            }"##).dispatch();

        assert_eq!(response.status(), Status::Ok);

        let response = response.into_string().unwrap();
        assert_eq!(response.contains(r##""id":0"##), true);
        assert_eq!(response.contains(r##""categoria":"Lanche""##), true);
        assert_eq!(response.contains(r##""ingredientes":["Pão","Hambúrguer"]"##), true);
    }

    #[test]
    fn test_update_produto() {
        let mut mock_produto_gateway = produto_gateway::MockProdutoGateway::new();
        mock_produto_gateway
            .expect_get_produto_by_id()
            .times(1)
            .returning(|_| Ok(create_valid_produto()));

        mock_produto_gateway
            .expect_update_produto()
            .times(1)
            .returning(|produto| Ok(produto));

        let produto_gateway: Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>> = Arc::new(Mutex::new(mock_produto_gateway));

        let rocket = rocket::build()
            .mount("/", routes())
            .manage(produto_gateway);

        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.put("/1")
            .header(ContentType::JSON)
            .body(r##"{
                "nome": "Novo Nome",
                "categoria": "Sobremesa",
                "preco": 15.70
            }"##).dispatch();

        assert_eq!(response.status(), Status::Ok);

        let response = response.into_string().unwrap();
        assert_eq!(response.contains(r##""nome":"Novo Nome""##), true);
        assert_eq!(response.contains(r##""categoria":"Sobremesa""##), true);
        assert_eq!(response.contains(r##""preco":15.7"##), true);
    }

    #[test]
    fn test_delete_produto() {
        let mut mock_produto_gateway = produto_gateway::MockProdutoGateway::new();
        mock_produto_gateway
            .expect_delete_produto()
            .times(1)
            .returning(|_| Ok(()));

        let produto_gateway: Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>> = Arc::new(Mutex::new(mock_produto_gateway));

        let rocket = rocket::build()
            .mount("/", routes())
            .manage(produto_gateway);

        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.delete("/1").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_handle_not_found() {
        let mut mock_produto_gateway = produto_gateway::MockProdutoGateway::new();
        mock_produto_gateway
            .expect_get_produto_by_id()
            .times(1)
            .returning(|_| Err(DomainError::NotFound));

        let produto_gateway: Arc<Mutex<dyn produto_gateway::ProdutoGateway + Sync + Send>> = Arc::new(Mutex::new(mock_produto_gateway));

        let rocket = rocket::build()
            .mount("/", routes())
            .register("/", catchers())
            .manage(produto_gateway);

        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/1").dispatch();

        assert_eq!(response.status(), Status::NotFound);

        let response = response.into_string().unwrap();
        assert_eq!(r##"{"msg":"Produto não encontrado!","status":404}"##, response);
    }
}
