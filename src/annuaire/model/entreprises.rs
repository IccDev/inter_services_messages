use common_crates::{
    serde::{self, Deserialize, Serialize}
};
use super::Adresse;


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct Entreprise {
    pub id: i32,
    pub nom: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct EntrepriseAdresse {
    pub id: i32,
    pub nom: String,
    pub description: Option<String>,
    pub adresse: Adresse
}