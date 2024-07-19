use common_crates::{
    serde::{self, Deserialize, Serialize}
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct AnnuaireLinkUserDiplomeMsg {
    pub profile_id: i32,
    pub diplome_ids: Vec<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct UserDiplome {
    pub profile_id: i32,
    pub diplome_id: i32
}