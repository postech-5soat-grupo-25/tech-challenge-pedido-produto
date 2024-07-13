use tech_challenge::api;

fn main() {
  match api::server::main() {
    Ok(_) => println!("Server finished"),
    Err(e) => println!("Error: {}", e),
  };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_main() {
    assert_eq!(1, 1);
  }
}
