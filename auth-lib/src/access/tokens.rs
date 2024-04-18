use serde::Deserialize;

/// Secret should parsed in environment
#[derive(Debug, Deserialize, Clone)]
pub struct Secret {
    pub refresh: String,
    pub access: String,
    pub salt: String,
}