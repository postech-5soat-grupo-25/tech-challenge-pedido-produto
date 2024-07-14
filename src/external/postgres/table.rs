use std::collections::HashMap;

#[derive(Clone)]
pub enum TablesNames {
  Pedido,
  Produto,
}

impl TablesNames {
  pub fn to_string(&self) -> String {
    match self {
      TablesNames::Pedido => "pedido".to_string(),
      TablesNames::Produto => "produto".to_string(),
    }
  }
}

#[derive(Clone)]
pub enum ColumnTypes {
  Index,
  Text,
  Integer,
  Float,
  Boolean,
  Timestamp,
  JSON,
  Char(usize),
  VARCHAR(usize),
  ENUM(String, Vec<String>),
}

impl ColumnTypes {
  pub fn to_string(&self) -> String {
    match self {
      ColumnTypes::Boolean => "BOOLEAN".to_string(),
      ColumnTypes::Float => "FLOAT".to_string(),
      ColumnTypes::Index => "SERIAL PRIMARY KEY".to_string(),
      ColumnTypes::Integer => "INTEGER".to_string(),
      ColumnTypes::Text => "TEXT".to_string(),
      ColumnTypes::Timestamp => "TIMESTAMP".to_string(),
      ColumnTypes::JSON => "JSON".to_string(),
      ColumnTypes::Char(size) => format!("CHAR({})", size),
      ColumnTypes::VARCHAR(size) => format!("VARCHAR({})", size),
      ColumnTypes::ENUM(name, values) => {
        let values_str = values.iter().map(|v| format!("'{}'", v)).collect::<Vec<_>>().join(", ");
        format!("{} ENUM({})", name, values_str)
    },
    }
  }
}
#[derive(Clone)]
pub struct ColumnNullable(pub bool);

impl ColumnNullable {
  pub fn to_string(&self) -> String {
    match self {
      ColumnNullable(true) => "NULL".to_string(),
      ColumnNullable(false) => "NOT NULL".to_string(),
    }
  }
}
#[derive(Clone)]
pub struct ColumnDefault(pub Option<String>);

impl ColumnDefault {
  pub fn to_string(&self) -> String {
    match self {
      ColumnDefault(Some(value)) => format!("DEFAULT {}", value),
      ColumnDefault(None) => "".to_string(),
    }
  }
}

#[derive(Clone)]
pub struct Table {
  pub name: TablesNames,
  pub columns: HashMap<String, (ColumnTypes, ColumnNullable, ColumnDefault)>
}

impl Table {
  pub fn get_create_if_not_exists_query(&self) -> String {
    let query = format!("CREATE TABLE IF NOT EXISTS public.{} (", self.name.to_string());
    let mut columns_query = String::new();
    self.columns.iter().for_each(|(column_name, (column_type, column_nullable, column_default))| {
      columns_query.push_str(&format!("{} {} ", column_name, column_type.to_string()));
      columns_query.push_str(&format!("{} ", column_nullable.to_string()));
      columns_query.push_str(&format!("{}, ", column_default.to_string()));
    });
    columns_query.pop();
    columns_query.pop();

    let query = format!("{}{})", query, columns_query);
    query
  }
}
