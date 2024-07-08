use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    base::{
        assertion_concern,
        domain_error::DomainError,
    },
    entities::ingredientes::Ingredientes,
};

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema, PartialEq)]
pub enum Categoria {
    Lanche,
    Bebida,
    Acompanhamento,
    Sobremesa,
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct Produto {
    id: usize,
    nome: String,
    foto: String,
    descricao: String,
    categoria: Categoria,
    preco: f64,
    ingredientes: Ingredientes,
    data_criacao: String,
    data_atualizacao: String,
}

impl Produto {
    pub fn new(
        id: usize,
        nome: String,
        foto: String,
        descricao: String,
        categoria: Categoria,
        preco: f64,
        ingredientes: Ingredientes,
        data_criacao: String,
        data_atualizacao: String,
    ) -> Self {
        Produto {
            id,
            nome,
            foto,
            descricao,
            categoria,
            preco,
            ingredientes,
            data_criacao,
            data_atualizacao,
        }
    }

    pub fn validate_entity(&self) -> Result<(), DomainError> {
        match self.categoria {
            Categoria::Lanche | Categoria::Acompanhamento | Categoria::Bebida => (),
            // | Categoria::Sobremesa => (),
            _ => {
                return Err(DomainError::Invalid(
                    "Categoria do Produto é inválida".to_string(),
                ))
            }
        };
        assertion_concern::assert_argument_not_empty(self.nome.clone())?;
        assertion_concern::assert_argument_not_empty(self.descricao.clone())?;
        assertion_concern::assert_argument_not_negative(self.preco.clone())?;
        assertion_concern::assert_argument_timestamp_format(self.data_criacao.clone())?;
        assertion_concern::assert_argument_timestamp_format(self.data_atualizacao.clone())?;
        Ok(())
    }

    // Getters
    pub fn id(&self) -> &usize {
        &self.id
    }

    pub fn nome(&self) -> &String {
        &self.nome
    }

    pub fn foto(&self) -> &String {
        &self.foto
    }

    pub fn descricao(&self) -> &String {
        &self.descricao
    }

    pub fn categoria(&self) -> &Categoria {
        &self.categoria
    }

    pub fn preco(&self) -> f64 {
        self.preco
    }

    pub fn ingredientes(&self) -> &Ingredientes {
        &self.ingredientes
    }

    pub fn data_criacao(&self) -> &String {
        &self.data_criacao
    }

    pub fn data_atualizacao(&self) -> &String {
        &self.data_atualizacao
    }

    // Setters
    pub fn set_nome(&mut self, nome: String) -> Result<(), DomainError> {
        assertion_concern::assert_argument_not_empty(nome.clone())?;
        self.nome = nome;
        Ok(())
    }

    pub fn set_foto(&mut self, foto: String) {
        self.foto = foto;
    }

    pub fn set_descricao(&mut self, descricao: String) -> Result<(), DomainError> {
        assertion_concern::assert_argument_not_empty(descricao.clone())?;
        self.descricao = descricao;
        Ok(())
    }

    pub fn set_categoria(&mut self, categoria: Categoria) {
        self.categoria = categoria;
    }

    pub fn set_preco(&mut self, preco: f64) -> Result<(), DomainError> {
        assertion_concern::assert_argument_not_negative(preco.clone())?;
        self.preco = preco;
        Ok(())
    }

    pub fn set_ingredientes(&mut self, ingredientes: Ingredientes) {
        self.ingredientes = ingredientes;
    }

    pub fn set_data_atualizacao(&mut self, data_atualizacao: String) -> Result<(), DomainError> {
        assertion_concern::assert_argument_timestamp_format(data_atualizacao.clone())?;
        self.data_atualizacao = data_atualizacao;
        Ok(())
    }
}

// Unit Tests
#[cfg(test)]
mod tests {
    use chrono::Utc;
    use super::*;

    fn create_valid_produto() -> Produto {
        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        Produto::new(
            1,
            "Cheeseburger".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            Categoria::Lanche,
            9.99,
            Ingredientes::new(vec![
                "Pão".to_string(),
                "Hambúrguer".to_string(),
                "Queijo".to_string(),
            ])
            .unwrap(),
            _now.clone(),
            _now,
        )
    }

    #[test]
    fn test_produto_creation_valid() {
        let produto = create_valid_produto();
        assert_eq!(produto.id(), &1);
        assert_eq!(produto.nome(), "Cheeseburger");
        assert_eq!(produto.foto(), "cheeseburger.png");
        assert_eq!(produto.descricao(), "O clássico pão, carne e queijo!");
        assert_eq!(produto.categoria(), &Categoria::Lanche);
        assert_eq!(produto.preco(), 9.99);
    }

    #[test]
    fn test_produto_validate_entity_valid() {
        let produto = create_valid_produto();
        assert!(produto.validate_entity().is_ok());
    }

    #[test]
    fn test_produto_validate_entity_empty_nome() {
        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        let produto = Produto::new(
            1,
            "".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            Categoria::Lanche,
            9.99,
            Ingredientes::new(vec![
                "Pão".to_string(),
                "Hambúrguer".to_string(),
                "Queijo".to_string(),
            ])
            .unwrap(),
            _now.clone(),
            _now,
        );
        let result = produto.validate_entity();
        assert!(
            matches!(result, Err(DomainError::Empty)),
            "Esperado Err(DomainError::Empty), obtido {:?}",
            result
        );
    }

    #[test]
    fn test_produto_validate_entity_negative_preco() {
        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        let produto = Produto::new(
            1,
            "Cheeseburger".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            Categoria::Lanche,
            -10.0,
            Ingredientes::new(vec![
                "Pão".to_string(),
                "Hambúrguer".to_string(),
                "Queijo".to_string(),
            ])
            .unwrap(),
            _now.clone(),
            _now,
        );
        let result = produto.validate_entity();
        assert!(
            matches!(result, Err(DomainError::NonPositive)),
            "Esperado Err(DomainError::NonPositive), obtido {:?}",
            result
        );
    }

    #[test]
    fn test_produto_setters_valid() {
        let mut produto = create_valid_produto();
        let _ = produto.set_nome("Salada Burger".to_string());
        let _ = produto.set_foto("salada_burguer.png".to_string());
        let _ = produto.set_descricao("Delicioso hambúrguer com salada fresca!".to_string());
        let _ = produto.set_preco(10.99);
        assert_eq!(produto.nome(), "Salada Burger");
        assert_eq!(produto.foto(), "salada_burguer.png");
        assert_eq!(
            produto.descricao(),
            "Delicioso hambúrguer com salada fresca!"
        );
        assert_eq!(produto.preco(), 10.99);
    }

    #[test]
    fn test_produto_set_categoria_valid() {
        let mut produto = create_valid_produto();
        produto.set_categoria(Categoria::Bebida);
        assert_eq!(produto.categoria(), &Categoria::Bebida);
    }

    #[test]
    fn test_produto_set_nome_empty() {
        let mut produto = create_valid_produto();
        let result = produto.set_nome("".to_string());
        assert!(
            matches!(result, Err(DomainError::Empty)),
            "Esperado Err(DomainError::Empty), obtido {:?}",
            result
        );
    }

    #[test]
    fn test_produto_set_preco_negative() {
        let mut produto = create_valid_produto();
        let result = produto.set_preco(-1.0);
        assert!(
            matches!(result, Err(DomainError::NonPositive)),
            "Esperado Err(DomainError::NonPositive), obtido {:?}",
            result
        );
    }

    #[test]
    fn test_produto_set_data_atualizacao_invalid_format() {
        let mut produto = create_valid_produto();
        let result = produto.set_data_atualizacao("18-02-2024".to_string());
        assert!(
            matches!(result, Err(DomainError::Invalid(_))),
            "Esperado Err(DomainError::Invalid), obtido {:?}",
            result
        );
    }
}
