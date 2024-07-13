use crate::base::domain_error::DomainError;
use rocket::http::Status;
use rocket::serde::json::Json;
use schemars::JsonSchema;
use serde::Serialize;

impl From<DomainError> for Status {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::AlreadyExists => Status::Conflict,
            DomainError::NotFound => Status::NotFound,
            DomainError::Empty => Status::BadRequest,
            DomainError::Unauthorized => Status::Unauthorized,
            DomainError::Invalid(_) => Status::BadRequest,
            _ => Status::InternalServerError,
        }
    }
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ErrorResponse {
    pub msg: String,
    pub status: usize,
}

#[catch(400)]
fn bad_request() -> Json<ErrorResponse> {
    let error = ErrorResponse {
        msg: "Input inválido".to_string(),
        status: 400,
    };
    Json(error)
}

#[catch(401)]
fn unauthorized() -> Json<ErrorResponse> {
    let error = ErrorResponse {
        msg: "Credenciais invalidas".to_string(),
        status: 401,
    };
    Json(error)
}

#[catch(500)]
fn internal() -> Json<ErrorResponse> {
    let error = ErrorResponse {
        msg: "Erro inesperado. Tente novamente mais tarde".to_string(),
        status: 500,
    };
    Json(error)
}

pub fn generic_catchers() -> Vec<rocket::Catcher> {
    catchers![bad_request, unauthorized, internal]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn test_empty() {
        #[get("/")]
        async fn route() -> Result<(), Status> {
            Err(DomainError::Empty.into())
        }

        let rocket = rocket::build()
            .mount("/", routes![route])
            .register("/", generic_catchers());

        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::BadRequest);

        let response = response.into_string().unwrap();
        assert_eq!(r##"{"msg":"Input inválido","status":400}"##, response);
    }

    #[test]
    fn test_invalid() {
        #[get("/")]
        async fn route() -> Result<(), Status> {
            Err(DomainError::Invalid("Entity".to_string()).into())
        }

        let rocket = rocket::build()
            .mount("/", routes![route])
            .register("/", generic_catchers());

        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::BadRequest);

        let response = response.into_string().unwrap();
        assert_eq!(r##"{"msg":"Input inválido","status":400}"##, response);
    }

    #[test]
    fn test_unauthorized() {
        #[get("/")]
        async fn route() -> Result<(), Status> {
            Err(DomainError::Unauthorized.into())
        }

        let rocket = rocket::build()
            .mount("/", routes![route])
            .register("/", generic_catchers());

        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Unauthorized);

        let response = response.into_string().unwrap();
        assert_eq!(r##"{"msg":"Credenciais invalidas","status":401}"##, response);
    }

    #[test]
    fn test_internal() {
        #[get("/")]
        async fn route() -> Result<(), Status> {
            Err(Status::InternalServerError)
        }

        let rocket = rocket::build()
            .mount("/", routes![route])
            .register("/", generic_catchers());

        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::InternalServerError);

        let response = response.into_string().unwrap();
        assert_eq!(r##"{"msg":"Erro inesperado. Tente novamente mais tarde","status":500}"##, response);
    }

    #[test]
    fn test_status_from_error() {
        assert_eq!(Status::from(DomainError::AlreadyExists), Status::Conflict);
        assert_eq!(Status::from(DomainError::NotFound), Status::NotFound);
        assert_eq!(Status::from(DomainError::Empty), Status::BadRequest);
        assert_eq!(Status::from(DomainError::Unauthorized), Status::Unauthorized);
        assert_eq!(Status::from(DomainError::Invalid("Entity".to_string())), Status::BadRequest);
        assert_eq!(Status::from(DomainError::NonPositive), Status::InternalServerError);
    }
}
