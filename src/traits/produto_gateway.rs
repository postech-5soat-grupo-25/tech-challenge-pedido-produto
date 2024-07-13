use mockall::*;

use crate::base::domain_error::DomainError;
use crate::entities::produto::{Produto, Categoria};
use std::fmt;
use std::str::FromStr;

impl FromStr for Categoria {
  type Err = ();

  fn from_str(input: &str) -> Result<Categoria, Self::Err> {
      match input {
          "Lanche" => Ok(Categoria::Lanche),
          "Acompanhamento" => Ok(Categoria::Acompanhamento),
          "Bebida" => Ok(Categoria::Bebida),
          "Sobremesa" => Ok(Categoria::Sobremesa),
          _ => Err(()),
      }
  }
}

impl fmt::Display for Categoria {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(
          f,
          "{}",
          match self {
              Categoria::Lanche => "Lanche",
              Categoria::Acompanhamento => "Acompanhamento",
              Categoria::Bebida => "Bebida",
              Categoria::Sobremesa => "Sobremesa",
          }
      )
  }
}

#[automock]
#[async_trait]
pub trait ProdutoGateway {
  async fn get_produtos(&self) -> Result<Vec<Produto>, DomainError>;

  async fn get_produto_by_id(&self, id: usize) -> Result<Produto, DomainError>;

  async fn create_produto(&mut self, produto: Produto) -> Result<Produto, DomainError>;

  async fn update_produto(&mut self, new_produto_data: Produto) -> Result<Produto, DomainError>;

  async fn delete_produto(&mut self, id: usize) -> Result<(), DomainError>;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_categoria_from_string() {
      let categoria = Categoria::from_str("Lanche").unwrap();
      assert_eq!(categoria, Categoria::Lanche);

      let categoria = Categoria::from_str("Acompanhamento").unwrap();
      assert_eq!(categoria, Categoria::Acompanhamento);

      let categoria = Categoria::from_str("Bebida").unwrap();
      assert_eq!(categoria, Categoria::Bebida);

      let categoria = Categoria::from_str("Sobremesa").unwrap();
      assert_eq!(categoria, Categoria::Sobremesa);
  }

  #[tokio::test]
  async fn test_categoria_to_string() {
      let categoria = Categoria::Lanche;
      assert_eq!(categoria.to_string(), "Lanche");

      let categoria = Categoria::Acompanhamento;
      assert_eq!(categoria.to_string(), "Acompanhamento");

      let categoria = Categoria::Bebida;
      assert_eq!(categoria.to_string(), "Bebida");

      let categoria = Categoria::Sobremesa;
      assert_eq!(categoria.to_string(), "Sobremesa");
  }
}