use common_crates::{
    serde::{self, Deserialize, Serialize}
};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct Entreprise {
    pub id: i32,
    pub nom: String,
    pub description: Option<String>,
    pub adresse_id: Option<i32>
}