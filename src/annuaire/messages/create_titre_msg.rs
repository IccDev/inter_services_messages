use common_crates::{
    serde::{self, Deserialize, Serialize}
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct AnnuaireCreateTitreMsg {
    pub nom: Option<String>,
    pub description: Option<String>
}