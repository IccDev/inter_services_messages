use common_crates::{
    serde::{self, Deserialize, Serialize}
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct AnnuaireLinkUserLangueMsg {
    pub profile_id: i32,
    pub langue_ids: Vec<i32>
}