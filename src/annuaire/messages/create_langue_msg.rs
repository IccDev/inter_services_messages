use common_crates::{
    serde::{self, Deserialize, Serialize}
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct AnnuaireCreateLangueMsg {
    pub nom: Option<String>,
    pub abbreviation: Option<String>
}