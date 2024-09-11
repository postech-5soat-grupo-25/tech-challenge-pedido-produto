use std::{env, str::FromStr};
use dotenv::dotenv;

#[derive(Debug, PartialEq, Clone)]
pub enum Env {
    Dev,
    Prod,
    Test,
}

impl FromStr for Env {
    type Err = ();

    fn from_str(input: &str) -> Result<Env, Self::Err> {
        match input {
            "dev" => Ok(Env::Dev),
            "prod" => Ok(Env::Prod),
            "test" => Ok(Env::Test),
            _ => Ok(Env::Dev),
        }
    }
}

impl ToString for Env {
    fn to_string(&self) -> String {
        match self {
            Env::Dev => "dev".to_string(),
            Env::Prod => "prod".to_string(),
            Env::Test => "test".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Config {
    pub env: String,
    pub db_url: String,
    pub api_key: String,
    pub rabbitmq_addr: Option<String>,
    pub queue_name: String,
}

impl Config {
    pub fn build() -> Config {
        dotenv().ok();
        let env = env::var("ENV").unwrap_or("test".to_string());
        let db_url = env::var("DB_URL")
            .unwrap_or("postgres://postgres:postgres@localhost:5432/postgres".to_string());
        let api_key = env::var("API_KEY").unwrap_or("api_key".to_string());
        let rabbitmq_addr = match env::var("RABBITMQ_ADDR"){
            Ok(addr) => Some(addr),
            Err(_) => None
        };

        let queue_name = env::var("QUEUE_NAME").unwrap_or("queue_name".to_string());

        Config {
            env,
            db_url,
            api_key,
            rabbitmq_addr,
            queue_name
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_from_str() {
        assert_eq!(Env::from_str("dev"), Ok(Env::Dev));
        assert_eq!(Env::from_str("prod"), Ok(Env::Prod));
        assert_eq!(Env::from_str("test"), Ok(Env::Test));
        assert_eq!(Env::from_str("invalid"), Ok(Env::Dev));
    }

    #[test]
    fn test_env_to_string() {
        assert_eq!(Env::Dev.to_string(), "dev".to_string());
        assert_eq!(Env::Prod.to_string(), "prod".to_string());
        assert_eq!(Env::Test.to_string(), "test".to_string());
    }

    #[test]
    fn test_config_build() {
        env::set_var("ENV", "dev");
        env::set_var("DB_URL", "postgres://postgres:postgres@localhost:5432/postgres");
        env::set_var("API_KEY", "valid_api_key");

        let config = Config::build();

        assert_eq!(config.env, "dev");
        assert_eq!(
            config.db_url,
            "postgres://postgres:postgres@localhost:5432/postgres".to_string()
        );
        assert_eq!(config.api_key, "valid_api_key".to_string());
    }
}
