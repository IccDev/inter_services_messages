use common_crates::{
    serde::{self, Deserialize, Serialize}
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct AnnuaireLinkUserProfessionMsg {
    pub profile_id: i32,
    pub domaine_id: i32,
    pub titre_id: i32,
    pub specialite_id: i32,
    pub entreprise_id: i32,
}