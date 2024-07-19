use common_crates::{
    serde::{self, Deserialize, Serialize}
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct AnnuaireLinkUserCompetenceMsg {
    pub profile_id: i32,
    pub competence_ids: Vec<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct UserCompetence {
    pub profile_id: i32,
    pub competence_id: i32
}