use common_crates::{
    serde::{self, Deserialize, Serialize}
};
use crate::annuaire::model::{
    EgliseDepartementsAdresse, Langue, Domaine, Titre, Specialite, 
    Diplome, Certificat, Competence, Profile, Contact,
    EcoleAdresse, EntrepriseAdresse, PlusInfos
};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct UserRespone {
    pub profile: Option<Profile>,
    pub eglise: Option<EgliseDepartementsAdresse>,
    pub contact: Option<Contact>,
    pub langues: Vec<Langue>,
    pub educations: Vec<Education>,
    pub professions: Vec<Profession>,
    pub diplomes: Vec<Diplome>,
    pub certificats: Vec<Certificat>,
    pub competences: Vec<Competence>,
    pub plusinfos: Option<PlusInfos>
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct Education {
    pub domaine: Option<Domaine>,
    pub titre: Option<Titre>,
    pub specialite: Option<Specialite>,
    pub ecole: Option<EcoleAdresse>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct Profession {
    pub domaine: Option<Domaine>,
    pub titre: Option<Titre>,
    pub specialite: Option<Specialite>,
    pub entreprise: Option<EntrepriseAdresse>
}
