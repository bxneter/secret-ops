mod dto;
use self::dto::{Secrets, ServiceToken};
use super::SecretOps;
use crate::error::{Error, Result};
use aes_gcm::aead::Aead;
use aes_gcm::aes::cipher::consts::U16;
use aes_gcm::aes::Aes256;
use aes_gcm::{AesGcm, KeyInit, Nonce};
use async_trait::async_trait;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;

const BASE_URL: &str = "https://app.infisical.com";

pub struct Infisical {
    client: Client,
    service_token_secret: String,
}

#[async_trait]
impl SecretOps for Infisical {
    fn new(service_token: &str) -> Result<Self> {
        let service_token_secret = service_token.split(".").last().ok_or(Error::InvalidToken)?.to_string();
        let bearer = HeaderValue::from_str(&format!("bearer {service_token}"))?;
        let authorization = (AUTHORIZATION, bearer);
        let headers = HeaderMap::from_iter([authorization]);
        let client = Client::builder().default_headers(headers).build()?;

        Ok(Self { client, service_token_secret })
    }

    async fn get_kv_secrets(&self) -> Result<Vec<(String, String)>> {
        let service_token = self.get_service_token().await?;
        let workspace_secret = self.decrypt(
            &service_token.encrypted_key,
            &service_token.iv,
            &service_token.tag,
            &self.service_token_secret,
        )?;

        self.get_secrets(&service_token.workspace, &service_token.environment)
            .await?
            .secrets
            .into_iter()
            .map(|secret| {
                Ok((
                    self.decrypt(&secret.key_ciphertext, &secret.key_iv, &secret.key_tag, &workspace_secret)?,
                    self.decrypt(&secret.value_ciphertext, &secret.value_iv, &secret.value_tag, &workspace_secret)?,
                ))
            })
            .collect()
    }
}

impl Infisical {
    async fn get_service_token(&self) -> Result<ServiceToken> {
        let url = format!("{BASE_URL}/api/v2/service-token");

        Ok(self.client.get(url).send().await?.json().await?)
    }

    async fn get_secrets(&self, workspace: &str, environment: &str) -> Result<Secrets> {
        let url = format!("{BASE_URL}/api/v3/secrets?environment={environment}&workspaceId={workspace}");

        Ok(self.client.get(url).send().await?.json().await?)
    }

    fn decrypt(&self, encrypted: &str, iv: &str, tag: &str, secret: &str) -> Result<String> {
        let cipher = AesGcm::<Aes256, U16>::new_from_slice(secret.as_bytes()).map_err(Error::InvalidKeyLength)?;
        let tag = STANDARD.decode(tag)?;
        let encrypted = STANDARD.decode(encrypted)?;
        let iv = STANDARD.decode(iv)?;
        let nonce = Nonce::<U16>::from_slice(&iv);
        let plaintext = cipher.decrypt(nonce, [encrypted, tag].concat().as_slice()).map_err(Error::DecryptErr)?;

        Ok(String::from_utf8(plaintext)?)
    }
}
