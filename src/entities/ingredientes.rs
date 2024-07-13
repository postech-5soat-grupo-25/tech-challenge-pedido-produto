use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::base::domain_error::DomainError;

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct Ingredientes(Vec<String>);

impl Ingredientes {
    pub fn new(ingredientes: Vec<String>) -> Result<Self, DomainError> {
        return Ok(Ingredientes(ingredientes));
    }

    pub fn to_vec_string(&self) -> Vec<String> {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ingredientes_valid() {
        let ingredientes = Ingredientes::new(vec![
            "Pão".to_string(),
            "Hambúrguer".to_string(),
            "Queijo".to_string(),
        ]);
        assert!(ingredientes.is_ok());

        assert!(ingredientes.unwrap().to_vec_string().len() == 3);
    }

    #[test]
    fn test_ingredientes_single_item() {
        let ingredientes = Ingredientes::new(vec!["Queijo".to_string()]);
        assert!(ingredientes.is_ok());
    }
}
