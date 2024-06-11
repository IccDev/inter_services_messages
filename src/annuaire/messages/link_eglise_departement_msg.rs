use common_crates::{
    serde::{self, Deserialize, Serialize}
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct AnnuaireLinkEgliseDepartMsg {
    pub eglise_id: i32,
    pub depart_id: i32
}