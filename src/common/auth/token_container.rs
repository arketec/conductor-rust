use serde::{ Deserialize, Serialize };
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TokenContainer {
    pub token: String,
}
