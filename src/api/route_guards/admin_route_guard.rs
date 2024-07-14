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
use crate::traits::user_group_validator_adapter::{UserGroup, UserGroupValidatorAdapter};

use std::sync::Arc;

pub struct AdminGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminGuard {
  type Error = DomainError;

  async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
      match req.headers().get_one("UserGroup") {
          Some(user_group_string) => {
              let user_group_validator = req
                  .rocket()
                  .state::<Arc<dyn UserGroupValidatorAdapter + Sync + Send>>()
                  .unwrap();
              match user_group_validator.validate_user_group(user_group_string.to_string(), UserGroup::Admin){
                  true => Outcome::Success(AdminGuard),
                  false => {
                      return Outcome::Error((Status::Unauthorized, DomainError::Invalid("Invalid User Group".to_string())))
                  }
              }
          }
          None => Outcome::Error((
              Status::BadRequest,
              DomainError::Invalid("Missing User Group".to_string()),
          )),
      }
  }
}

impl<'a> OpenApiFromRequest<'a> for AdminGuard {
  fn from_request_input(
      _gen: &mut OpenApiGenerator,
      _name: String,
      _required: bool,
  ) -> Result<RequestHeaderInput, OpenApiError> {
      let security_scheme = SecurityScheme {
          description: Some("Usuário Admin para acessar.".to_owned()),

          data: SecuritySchemeData::ApiKey {
              name: "UserGroup".to_owned(),
              location: "header".to_owned(),
          },
          extensions: Object::default(),
      };

      let mut security_req = SecurityRequirement::new();
      security_req.insert("UserGroup".to_owned(), Vec::new());
      Ok(RequestHeaderInput::Security(
          "UserGroup".to_owned(),
          security_scheme,
          security_req,
      ))
  }
}
