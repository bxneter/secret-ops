mod error;
#[cfg(feature = "infisical")]
pub mod infisical;
pub use crate::error::{Error, Result};
use async_trait::async_trait;
use serde::de::DeserializeOwned;

#[async_trait]
pub trait SecretOps {
    fn new(token: &str) -> Result<Self>
    where
        Self: Sized;

    fn from_env() -> Result<Self>
    where
        Self: Sized,
    {
        Self::new(&std::env::var("SECRET_OPS_TOKEN").or(Err(Error::EnvNotPresent))?)
    }

    async fn get_kv_secrets(&self) -> Result<Vec<(String, String)>>;

    async fn inject(self) -> Result<Self>
    where
        Self: Sized + Sync,
    {
        for (key, value) in self.get_kv_secrets().await? {
            std::env::set_var(key, value);
        }

        Ok(self)
    }

    fn deserialize<T: DeserializeOwned>(&self) -> Result<T> {
        Ok(envy::from_env()?)
    }
}
