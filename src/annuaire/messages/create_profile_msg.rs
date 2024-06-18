use common_crates::{
    serde::{self, Deserialize, Serialize}
};

use crate::model::*;
use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct AnnuaireCreateProfileMsg {
    pub nom: Option<String>,
    pub prenom: Option<String>,
    pub photo: Option<String>,
    pub star: Option<bool>,
    pub adresse: Option<Adresse>,
    pub gsm: Option<String>,
    pub email: Option<String>,
    pub consentement_nom: Option<bool>,
    pub consentement_gsm: Option<bool>,
    pub consentement_email: Option<bool>,
    pub consentement_ecole: Option<bool>,
    pub consentement_diplome: Option<bool>,
    pub consentement_certificat: Option<bool>,
    pub consentement_entreprise: Option<bool>,
    pub consentement_adresse: Option<bool>,
    pub eglise: Option<EgliseDepart>,
    pub langues: Vec<i32>,
    pub educations_ids: Vec<EducationIds>,
    pub professions_ids: Vec<ProfessionIds>,
    pub diplomes_ids: Vec<i32>,
    pub educations: Vec<Education>,
    pub professions: Vec<Profession>,
    pub diplomes: Vec<AnnuaireCreateDiplomeMsg>,
    pub certificats: Vec<AnnuaireCreateCertificatMsg>,
    pub competences: Vec<AnnuaireCreateCompetenceMsg>,
    pub plus_infos: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct EgliseDepart {
    pub eglise: i32,
    pub departements: Vec<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct EducationIds {
    pub domaine_id: i32,
    pub titre_id: i32,
    pub specialite_id: i32,
    pub ecole_id: i32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct Education {
    pub domaine: Option<AnnuaireCreateDomaineMsg>,
    pub titre: Option<AnnuaireCreateTitreMsg>,
    pub specialite: Option<AnnuaireCreateSpecialiteMsg>,
    pub ecole: Option<AnnuaireCreateEcoleMsg>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct ProfessionIds {
    pub domaine_id: i32,
    pub titre_id: i32,
    pub specialite_id: i32,
    pub entreprise_id: i32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct Profession {
    pub domaine: Option<AnnuaireCreateDomaineMsg>,
    pub titre: Option<AnnuaireCreateTitreMsg>,
    pub specialite: Option<AnnuaireCreateSpecialiteMsg>,
    pub entreprise: Option<AnnuaireCreateEntrepriseMsg>
}