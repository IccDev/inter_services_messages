use common_crates::{
    serde::{self, Deserialize, Serialize}
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct AnnuaireLinkUserCertificatMsg {
    pub profile_id: i32,
    pub certificat_ids: Vec<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct UserCertificat {
    pub profile_id: i32,
    pub certificat_id: i32
}