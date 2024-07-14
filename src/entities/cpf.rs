use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use regex::Regex;

use crate::base::domain_error::DomainError;

#[derive(Clone, Deserialize, Debug, JsonSchema, Serialize)]
pub struct Cpf(pub String);

impl Cpf {
    pub fn new(codigo: String) -> Result<Self, DomainError> {
        if codigo.is_empty() {
            return Err(DomainError::Empty);
        }
        // Default admin user
        if codigo == "000.000.000-00" {
            return Ok(Cpf("00000000000".to_string()));
        }
        let regex_pattern = Regex::new(r"^\d{3}\.\d{3}\.\d{3}-\d{2}$|^\d{11}$").unwrap();
        if regex_pattern.is_match(&codigo) {
            let codigo = Cpf::keep_only_numbers(codigo);
            if Cpf::validate(codigo.clone()) {
                Ok(Cpf(codigo))
            } else {
                Err(DomainError::Invalid("CPF".to_string()))
            }
        } else {
            Err(DomainError::Invalid("CPF".to_string()))
        }
    }

    fn keep_only_numbers(codigo: String) -> String {
        // let cpf = codigo;
        let codigo = codigo.replace(".", "");
        let codigo = codigo.replace("-", "");
        codigo
    }

    fn validate(codigo: String) -> bool {
        let cpf = codigo.chars().map(|d| d.to_digit(10).unwrap()).collect::<Vec<u32>>();
        let dv1 = (0..9).map(|i| cpf[i] * (10 - i as u32)).sum::<u32>() % 11;
        let dv1 = if dv1 < 2 { 0 } else { 11 - dv1 };
        let dv2 = (0..10).map(|i| cpf[i] * (11 - i as u32)).sum::<u32>() % 11;
        let dv2 = if dv2 < 2 { 0 } else { 11 - dv2 };
        dv1 == cpf[9] && dv2 == cpf[10]
    }

    pub fn get_string(&self) -> String {
        self.0.clone()
    }
}

impl PartialEq for Cpf {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpf_admin() {
        let cpf = Cpf::new("000.000.000-00".to_string());
        assert!(cpf.is_ok());

        let cpf = cpf.unwrap();

        assert_eq!(cpf.get_string(), "00000000000");
    }
    #[test]
    fn test_cpf_valid() {
        let cpf = Cpf::new("097.855.456-60".to_string());
        assert!(cpf.is_ok());

        let cpf = cpf.unwrap();

        assert_eq!(cpf.get_string(), "09785545660");
    }
    #[test]
    fn test_cpf_invalid_number() {
        let cpf = Cpf::new("000.000.000-01".to_string());
        assert!(cpf.is_err());
    }
    #[test]
    fn test_cpf_invalid_value() {
        let cpf = Cpf::new("wrong".to_string());
        assert!(cpf.is_err());
    }
}
