use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
};
use rocket_okapi::{
    gen::OpenApiGenerator,
    okapi::openapi3::{Object, SecurityRequirement, SecurityScheme, SecuritySchemeData},
    request::{OpenApiFromRequest, RequestHeaderInput},
    OpenApiError,
};

use crate::base::domain_error::DomainError;

use crate::traits::api_key_validator_adapter::ApiKeyValidatorAdapter;
use std::sync::Arc;

pub struct ApiKeyGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKeyGuard {
    type Error = DomainError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.headers().get_one("api-secret") {
            Some(key) => {
                let api_key_validator = req
                    .rocket()
                    .state::<Arc<dyn ApiKeyValidatorAdapter + Sync + Send>>()
                    .unwrap();
                match api_key_validator.validate_key(key.to_string()) {
                    true => Outcome::Success(ApiKeyGuard),
                    false => {
                        return Outcome::Error((Status::Unauthorized, DomainError::Invalid("API key".to_string())))
                    }
                }
            }
            None => Outcome::Error((
                Status::BadRequest,
                DomainError::Invalid("API key necessária".to_string()),
            )),
        }
    }
}

impl<'a> OpenApiFromRequest<'a> for ApiKeyGuard {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> Result<RequestHeaderInput, OpenApiError> {
        let security_scheme = SecurityScheme {
            description: Some("API necessária para acessar.".to_owned()),

            data: SecuritySchemeData::ApiKey {
                name: "api-secret".to_owned(),
                location: "header".to_owned(),
            },
            extensions: Object::default(),
        };

        let mut security_req = SecurityRequirement::new();
        security_req.insert("ApiKeyAuth".to_owned(), Vec::new());
        Ok(RequestHeaderInput::Security(
            "ApiKeyAuth".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}
