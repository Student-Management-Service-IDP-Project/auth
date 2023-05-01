use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Secret {
    pub refresh: String,
    pub access: String,
}