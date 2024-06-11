use common_crates::{
    serde::{self, Deserialize, Serialize}
};
use crate::annuaire::model::{EgliseDepartements, Langue, Domaine, Titre, Specialite, 
        Diplome, Certificat, Competence, Ecole, Entreprise
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct AnnuaireAllInfosToCreateUserRespone {
    pub eglises: Vec<EgliseDepartements>,
    pub langues: Vec<Langue>,
    pub domaines: Vec<Domaine>,
    pub titres: Vec<Titre>,
    pub specialites: Vec<Specialite>,
    pub diplomes: Vec<Diplome>,
    pub certificats: Vec<Certificat>,
    pub competences: Vec<Competence>,
    pub ecoles: Vec<Ecole>,
    pub entreprises: Vec<Entreprise>
}

