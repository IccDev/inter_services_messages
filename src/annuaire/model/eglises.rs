use common_crates::{
    serde::{self, Deserialize, Serialize}
};
use super::{Departement, Adresse};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct Eglise {
    pub id: i32,
    pub nom: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct EgliseDepartements {
    pub id: i32,
    pub nom: String,
    pub description: Option<String>,
    pub departements: Vec<Departement>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct EgliseDepartementsAdresse {
    pub id: i32,
    pub nom: String,
    pub description: Option<String>,
    pub departements: Vec<Departement>,
    pub adresse: Adresse
}