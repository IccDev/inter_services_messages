use common_crates::{
    serde::{self, Deserialize, Serialize}
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct AnnuaireLinkUserPluseInfosMsg {
    pub profile_id: i32,
    pub plusinfos_id: i32
}