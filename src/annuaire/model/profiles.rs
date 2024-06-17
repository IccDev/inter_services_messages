use common_crates::{
    serde::{self, Deserialize, Serialize}
};
use super::Adresse;


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct Profile {
    pub id: i32,
    pub nom: String,
    pub prenom: String,
    pub photo: Option<String>,
    pub star: bool,
    pub adresse: Option<Adresse>
}