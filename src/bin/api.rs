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
