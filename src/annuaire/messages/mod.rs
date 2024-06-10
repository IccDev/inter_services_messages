mod search_msg;
mod create_eglise_msg;

pub use search_msg::*;
pub use create_eglise_msg::*;
use common_crates::{
    serde::{self, Deserialize, Serialize}
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub enum AnnuaireMsg {
    Search(AnnuaireSearchMsg),
    CreateEglises(Vec<AnnuaireCreateEgliseMsg>),
    /*CreateUser(User),
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
