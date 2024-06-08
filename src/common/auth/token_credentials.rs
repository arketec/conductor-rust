use serde::{ Deserialize, Serialize };
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TokenCredentials {
    #[serde(rename = "keyId")]
    pub key_id: String,
    #[serde(rename = "keySecret")]
    pub key_secret: String,
}
