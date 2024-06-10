use common_crates::{
    serde::{self, Deserialize, Serialize}
};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct AnnuaireSearchRespone {
    pub data: Vec<String>
}

