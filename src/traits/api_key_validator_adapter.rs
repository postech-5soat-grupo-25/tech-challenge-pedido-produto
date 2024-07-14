use mockall::*;

#[automock]
#[async_trait]
pub trait ApiKeyValidatorAdapter {
    fn validate_key(&self, comming_api_key: String) -> bool;
}