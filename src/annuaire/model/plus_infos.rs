use common_crates::{
    serde::{self, Deserialize, Serialize}
};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct PlusInfos {
    pub id: i32,
    pub description: Option<String>,
}