mod search_response;

pub use search_response::*;
use common_crates::{
    serde::{self, Deserialize, Serialize}
};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub enum AnnuaireResponse {
    Search(AnnuaireSearchRespone),
    Create(i32)
}

