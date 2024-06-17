use common_crates::{
    serde::{self, Deserialize, Serialize}
};

use super::Adresse;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct Ecole {
    pub id: i32,
    pub nom: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct EcoleAdresse {
    pub id: i32,
    pub nom: String,
    pub description: Option<String>,
    pub adresse: Adresse
}