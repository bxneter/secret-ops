use secret_ops::toml::Toml;
use secret_ops::SecretOps;
use serde::Deserialize;

const TOML: &str = r#"
[http]
port = "80"
"#;

#[derive(Debug, Deserialize)]
struct Config {
    http_port: String,
}

#[tokio::main]
async fn main() {
    let config = Toml::new(TOML).unwrap().inject().await.unwrap().deserialize::<Config>().unwrap();

    println!("HTTP_PORT={}", config.http_port);
}
