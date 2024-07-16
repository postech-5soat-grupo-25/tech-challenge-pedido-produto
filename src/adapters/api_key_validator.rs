use crate::traits::api_key_validator_adapter::ApiKeyValidatorAdapter;

pub struct ApiKeyValidator {
    api_key: String,
}

impl ApiKeyValidator {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl ApiKeyValidatorAdapter for ApiKeyValidator {
    fn validate_key(&self, comming_api_key: String) -> bool {
        self.api_key == comming_api_key
    }
}

unsafe impl Sync for ApiKeyValidator {}
unsafe impl Send for ApiKeyValidator {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_key() {
        let api_key = "valid_api_key".to_string();
        let api_key_validator = ApiKeyValidator::new(api_key);

        assert_eq!(api_key_validator.validate_key("valid_api_key".to_string()), true);
        assert_eq!(api_key_validator.validate_key("invalid".to_string()), false);
    }
}