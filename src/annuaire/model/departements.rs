use common_crates::{
    serde::{self, Deserialize, Serialize}
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct Departement {
    pub id: i32,
    pub nom: String,
    pub abbreviation: Option<String>,
    pub description: Option<String>,
}
