use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    base::{assertion_concern, domain_error::DomainError},
    entities::{cpf::Cpf, produto::Produto},
};

// Considerar Ordem de Status
// Pendente => Pago => EmPreparacao => Pronto => Finalizado => (Cancelado)
// Cancelado em qualquer ponto
#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema, PartialEq)]
pub enum Status {
    EmPreparacao,
    Pago,
    Pronto,
    Pendente,
    Finalizado,
    Cancelado,
    Invalido,
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct Pedido {
    id: usize,
    cliente: Option<Cpf>,
    lanche: Option<Produto>,
    acompanhamento: Option<Produto>,
    bebida: Option<Produto>,
    pagamento: Option<String>,
    status: Status,
    data_criacao: String,
    data_atualizacao: String,
}

impl Pedido {
    pub fn new(
        id: usize,
        cliente: Option<Cpf>,
        lanche: Option<Produto>,
        acompanhamento: Option<Produto>,
        bebida: Option<Produto>,
        pagamento: Option<String>,
        status: Status,
        data_criacao: String,
        data_atualizacao: String,
    ) -> Self {
        Pedido {
            id,
            cliente,
            lanche,
            acompanhamento,
            bebida,
            pagamento,
            status,
            data_criacao,
            data_atualizacao,
        }
    }

    pub fn validate_entity(&self) -> Result<(), DomainError> {
        if self.lanche.is_none() && self.acompanhamento.is_none() && self.bebida.is_none() {
            return Err(DomainError::Invalid(
                "Pedido deve conter pelo menos um item entre Lanche, Acompanhamento ou Bebida"
                    .to_string(),
            ));
        };

        assertion_concern::assert_argument_timestamp_format(self.data_criacao.clone())?;
        assertion_concern::assert_argument_timestamp_format(self.data_atualizacao.clone())?;
        Ok(())
    }

    // Getters
    pub fn id(&self) -> &usize {
        &self.id
    }

    pub fn cliente(&self) -> Option<&Cpf> {
        self.cliente.as_ref()
    }

    pub fn lanche(&self) -> Option<&Produto> {
        self.lanche.as_ref()
    }

    pub fn acompanhamento(&self) -> Option<&Produto> {
        self.acompanhamento.as_ref()
    }

    pub fn bebida(&self) -> Option<&Produto> {
        self.bebida.as_ref()
    }

    pub fn pagamento(&self) -> Option<&String> {
        self.pagamento.as_ref()
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn data_criacao(&self) -> &String {
        &self.data_criacao
    }

    pub fn data_atualizacao(&self) -> &String {
        &self.data_atualizacao
    }

    pub fn valor_total(&self) -> f64 {
        let valor_lanche = match self.lanche() {
            Some(produto) => produto.preco(),
            None => 0.0,
        };

        let valor_acompanhamento = match self.acompanhamento() {
            Some(produto) => produto.preco(),
            None => 0.0,
        };

        let valor_bebida = match self.bebida() {
            Some(produto) => produto.preco(),
            None => 0.0,
        };

        valor_lanche + valor_bebida + valor_acompanhamento
    }

    // Setters
    pub fn set_pagamento(&mut self, pagamento: String) {
        self.pagamento = Some(pagamento);
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
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
    use super::*;
    use crate::entities::ingredientes::Ingredientes;
    use crate::entities::produto::Categoria;
    use crate::entities::produto::Produto;

    fn create_valid_produto(categoria: Categoria) -> Produto {
        let _now = "2021-08-01 00:00:00.000+0000".to_string();
        Produto::new(
            1,
            "Cheeseburger".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            categoria,
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

    fn create_valid_pedido() -> Pedido {
        let _now = "2021-08-01 00:00:00.000+0000".to_string();
        let cliente = Cpf::new("123.456.789-09".to_string()).unwrap();
        let produto = create_valid_produto(Categoria::Lanche);
        Pedido::new(
            1,
            Some(cliente),
            Some(produto),
            None,
            None,
            None,
            Status::Pendente,
            _now.clone(),
            _now,
        )
    }

    #[test]
    fn test_pedido_creation_valid() {
        let pedido = create_valid_pedido();
        assert_eq!(pedido.id(), &1);
        assert!(pedido.lanche().is_some());
        assert!(pedido.acompanhamento().is_none());
        assert!(pedido.bebida().is_none());
        assert_eq!(pedido.pagamento(), None);
        assert_eq!(pedido.status(), &Status::Pendente);
        assert_eq!(pedido.cliente().unwrap().get_string(), "12345678909".to_string());
        assert_eq!(pedido.data_criacao(), &"2021-08-01 00:00:00.000+0000".to_string());
        assert_eq!(pedido.data_atualizacao(), &"2021-08-01 00:00:00.000+0000".to_string());
    }

    #[test]
    fn test_pedido_validate_entity_valid() {
        let pedido = create_valid_pedido();
        assert!(pedido.validate_entity().is_ok());
    }

    #[test]
    fn test_pedido_validate_entity_no_items() {
        let _now = "2021-08-01 00:00:00.000+0000".to_string();
        let cliente = Cpf::new("123.456.789-09".to_string()).unwrap();
        let pedido = Pedido::new(
            1,
            Some(cliente),
            None,
            None,
            None,
            None,
            Status::Pendente,
            _now.clone(),
            _now,
        );
        let result = pedido.validate_entity();
        assert!(
            matches!(result, Err(DomainError::Invalid(_))),
            "Esperado Err(DomainError::Invalid), obtido {:?}",
            result
        );
    }

    #[test]
    fn test_pedido_set_data_atualizacao_invalid_format() {
        let mut pedido = create_valid_pedido();
        let result = pedido.set_data_atualizacao("18-02-2024".to_string());
        assert!(
            matches!(result, Err(DomainError::Invalid(_))),
            "Esperado Err(DomainError::Invalid), obtido {:?}",
            result
        );
    }

    #[test]
    fn test_pedido_invalid_creation_date() {
        let _now = "2021-08-01 00:00:00.000+0000".to_string();
        let cliente = Cpf::new("123.456.789-09".to_string()).unwrap();
        let pedido = Pedido::new(
            1,
            Some(cliente),
            None,
            None,
            None,
            None,
            Status::Pendente,
            "18-02-2024".to_string(),
            _now,
        );
        let result = pedido.validate_entity();
        assert!(
            matches!(result, Err(DomainError::Invalid(_))),
            "Esperado Err(DomainError::Invalid), obtido {:?}",
            result
        );
    }

    #[test]
    fn test_pedido_invalid_update_date() {
        let _now = "2021-08-01 00:00:00.000+0000".to_string();
        let cliente = Cpf::new("123.456.789-09".to_string()).unwrap();
        let pedido = Pedido::new(
            1,
            Some(cliente),
            None,
            None,
            None,
            None,
            Status::Pendente,
            _now,
            "18-02-2024".to_string(),
        );
        let result = pedido.validate_entity();
        assert!(
            matches!(result, Err(DomainError::Invalid(_))),
            "Esperado Err(DomainError::Invalid), obtido {:?}",
            result
        );
    }

    #[test]
    fn test_soma_valor_total_pedido() {
        let _now = "2021-08-01 00:00:00.000+0000".to_string();
        let cliente = Cpf::new("123.456.789-09".to_string()).unwrap();
        let lanche = create_valid_produto(Categoria::Lanche);
        let acompanhamento = create_valid_produto(Categoria::Acompanhamento);
        let bebida = create_valid_produto(Categoria::Bebida);
        let pedido = Pedido::new(
            1,
            Some(cliente),
            Some(lanche),
            Some(acompanhamento),
            Some(bebida),
            None,
            Status::Pendente,
            _now.clone(),
            _now,
        );
        assert_eq!(pedido.valor_total(), 29.97);
    }

    #[test]
    fn test_soma_valor_total_pedido_apenas_lanche() {
        let _now = "2021-08-01 00:00:00.000+0000".to_string();
        let cliente = Cpf::new("123.456.789-09".to_string()).unwrap();
        let lanche = create_valid_produto(Categoria::Lanche);
        let pedido = Pedido::new(
            1,
            Some(cliente),
            Some(lanche),
            None,
            None,
            None,
            Status::Pendente,
            _now.clone(),
            _now,
        );
        assert_eq!(pedido.valor_total(), 9.99);
    }

    #[test]
    fn test_pedido_set_pagamento() {
        let mut pedido = create_valid_pedido();

        assert_eq!(pedido.pagamento(), None);

        pedido.set_pagamento("pagamento_uuid".to_string());

        assert_eq!(pedido.pagamento(), Some(&"pagamento_uuid".to_string()));
    }

    #[test]
    fn test_pedido_set_status() {
        let mut pedido = create_valid_pedido();

        assert_eq!(pedido.pagamento(), None);

        pedido.set_status(Status::Pago);

        assert_eq!(pedido.status(), &Status::Pago);
    }

    #[test]
    fn test_pedido_set_data_atualizacao() {
        let mut pedido = create_valid_pedido();

        assert_eq!(pedido.data_atualizacao(), &"2021-08-01 00:00:00.000+0000".to_string());

        pedido.set_data_atualizacao("2021-10-01 00:00:00.000+0000".to_string()).unwrap();

        assert_eq!(pedido.data_atualizacao(), &"2021-10-01 00:00:00.000+0000".to_string());
    }
}
