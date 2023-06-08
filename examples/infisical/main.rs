use secret_ops::infisical::Infisical;
use secret_ops::SecretOps;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    key: String,
}

#[tokio::main]
async fn main() {
    let _ = std::env::set_var("SECRET_OPS_TOKEN", "<INFISICAL_SERVICE_TOKEN>");
    let config = Infisical::from_env().unwrap().inject().await.unwrap().deserialize::<Config>().unwrap();

    println!("KEY={}", config.key);
}
