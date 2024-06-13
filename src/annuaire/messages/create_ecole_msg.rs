use common_crates::{
    serde::{self, Deserialize, Serialize}
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct AnnuaireCreateEcoleMsg {
    pub nom: Option<String>,
    pub description: Option<String>,
    pub pays: Option<String>,
    pub ville: Option<String>,
    pub commune: Option<String>,
    pub code_postal: Option<String>,
    pub rue: Option<String>,
    pub numero: Option<i32>,
    pub boite: Option<i32>
}