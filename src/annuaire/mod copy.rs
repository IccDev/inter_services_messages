mod model;

pub use model::*;
use common_crates::{
    serde::{self, Deserialize, Serialize}
};
use std::hash::{Hash, Hasher};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub enum AnnuaireMessage {
    Search(AnnuaireSearchInput),
    /*CreateUser(User),
    CreateCampus(Campus),
    CreateCompetences(Competence),
    CreateDepartement(Departement),
    CreateDiplomes(DiplomeCertificat),
    CreateDomaine(Domaine),
    CreateEcole(Ecole),
    CreateEntreprise(Entreprise),
    CreateLangue(Langue),
    CreateLocalite(Localite),
    CreateSpecialite(Specialite),
    CreateTitre(Titre),
    CreateUserInfo(UserPlusInfos),*/
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub enum AnnuaireResponse {
    Search(AnnuaireSearchOutput),
    Create(i32)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct AnnuaireSearchInput {
    pub key: Option<String>,
    pub church: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct AnnuaireSearchOutput {
    pub data: Vec<User>
}

