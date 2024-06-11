use common_crates::{
    serde::{self, Deserialize, Serialize}
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct Specialite {
    pub id: i32,
    pub nom: String,
    pub description: Option<String>,
}