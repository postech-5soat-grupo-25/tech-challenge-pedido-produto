use tech_challenge::api;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
  let server = api::server::main().await;

  match server.launch().await {
    Ok(_) => {
      println!("Server started");
      Ok(())
    },
    Err(e) => {
      print!("Error starting server: {:?}", e);
      Err(e)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::{env, thread};

  #[tokio::test]
  async fn test_api() {
    test_api_works().await;
    test_api_breaks().await;
  }

  async fn test_api_works() {
    env::set_var("ENV", "test");

    thread::spawn(|| {
      let _rocket = main();
    });

    let client = reqwest::Client::new();

    let response = client.get("http://localhost:3000/pedidos").send().await.unwrap();

    assert_eq!(response.status(), 200);
  }

  async fn test_api_breaks() {
    env::set_var("ENV", "prod");

    thread::spawn(|| {
      let rocket = main();
      assert!(rocket.is_err());
    });
  }
}
