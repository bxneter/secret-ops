use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceToken {
    pub name: String,
    pub environment: String,
    pub workspace: String,
    pub encrypted_key: String,
    pub iv: String,
    pub tag: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Secrets {
    pub secrets: Vec<Secret>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Secret {
    pub version: u32,
    pub workspace: String,
    pub r#type: String,
    pub tags: Vec<String>,
    pub environment: String,
    pub algorithm: String,
    pub key_encoding: String,
    pub path: String,
    pub folder: String,

    #[serde(rename = "secretKeyCiphertext")]
    pub key_ciphertext: String,
    #[serde(rename = "secretKeyIV")]
    pub key_iv: String,
    #[serde(rename = "secretKeyTag")]
    pub key_tag: String,

    #[serde(rename = "secretValueCiphertext")]
    pub value_ciphertext: String,
    #[serde(rename = "secretValueIV")]
    pub value_iv: String,
    #[serde(rename = "secretValueTag")]
    pub value_tag: String,

    #[serde(rename = "secretCommentCiphertext")]
    pub comment_ciphertext: String,
    #[serde(rename = "secretCommentIV")]
    pub comment_iv: String,
    #[serde(rename = "secretCommentTag")]
    pub comment_tag: String,
}
