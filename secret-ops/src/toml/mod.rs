use super::SecretOps;
use crate::Result;
use async_trait::async_trait;
use toml::Value;

pub struct Toml {
    toml: Value,
}

#[async_trait]
impl SecretOps for Toml {
    fn new(toml_str: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self { toml: toml::from_str::<toml::Value>(toml_str)? })
    }

    fn from_env() -> Result<Self>
    where
        Self: Sized,
    {
        unimplemented!()
    }

    async fn get_kv_secrets(&self) -> Result<Vec<(String, String)>> {
        Ok(self.toml.as_table().map(|table| self.get_kv_from_table(None, table)).unwrap_or(vec![]))
    }
}

impl Toml {
    fn get_kv_from_table(&self, prefix: Option<String>, table: &toml::Table) -> Vec<(String, String)> {
        table
            .into_iter()
            .flat_map(|(key, value)| {
                let key = prefix.clone().map_or_else(|| key.clone(), |prefix| format!("{}_{}", prefix, key));
                match value {
                    toml::Value::String(string) => vec![(key.to_uppercase(), string.to_owned())],
                    toml::Value::Table(table) => self.get_kv_from_table(Some(key), table),
                    value => panic!("Unimplemented handler for type: {:?}", value),
                }
            })
            .collect()
    }
}
