pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The SECRET_OPS_TOKEN was not present in the current environment")]
    EnvNotPresent,

    #[error("Invalid token")]
    InvalidToken,

    #[error("{0}")]
    FromUtf8Err(#[from] std::string::FromUtf8Error),

    #[error("{0}")]
    EnvyErr(#[from] envy::Error),

    #[cfg(feature = "infisical")]
    #[error("Failed to{0}")]
    TokenToHeaderErr(#[from] reqwest::header::InvalidHeaderValue),

    #[cfg(feature = "infisical")]
    #[error("{0}")]
    ReqwestErr(#[from] reqwest::Error),

    #[cfg(feature = "infisical")]
    #[error("Base64 decode error: {0}")]
    Base64DecodeErr(#[from] base64::DecodeError),

    #[cfg(feature = "infisical")]
    #[error("Invalid key length: {0}")]
    InvalidKeyLength(aes_gcm::aes::cipher::InvalidLength),

    #[cfg(feature = "infisical")]
    #[error("Decrypt error: {0}")]
    DecryptErr(aes_gcm::Error),
}
