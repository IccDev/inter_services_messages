use common_crates::{
    serde::{self, Deserialize, Serialize}
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct Adresse {
    pub id: i32,
    pub pays: String,
    pub ville: String,
    pub commune: Option<String>,
    pub code_postal: Option<String>,
    pub rue: Option<String>,
    pub numero: Option<i32>,
    pub boite: Option<i32>,
}
