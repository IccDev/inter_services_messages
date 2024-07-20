use common_crates::{
    serde::{self, Deserialize, Serialize}
};
use super::Adresse;


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct Profile {
    pub id: i32,
    pub nom: String,
    pub prenom: String,
    pub photo: Option<String>,
    pub star: bool,
    pub adresse: Option<Adresse>
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct UserProfile {
    pub nom: Option<String>,
    pub prenom: Option<String>,
    pub photo: Option<String>,
    pub star: Option<bool>,
    pub adresse_id: Option<i32>,
    pub contact_id: Option<i32>,
    pub eglise_id: Option<String>,
    pub consentement_nom: Option<bool>,
    pub consentement_gsm: Option<bool>,
    pub consentement_email: Option<bool>,
    pub consentement_ecole: Option<bool>,
    pub consentement_diplome: Option<bool>,
    pub consentement_certificat: Option<bool>,
    pub consentement_entreprise: Option<bool>,
    pub consentement_adresse: Option<bool>,
}
