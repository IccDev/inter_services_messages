use common_crates::{
    serde::{self, Deserialize, Serialize}
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct Contact {
    pub id: i32,
    pub gsm: String,
    pub email: String,
}
