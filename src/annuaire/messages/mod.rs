mod search_msg;
mod create_eglise_msg;
mod create_departement_msg;
mod create_langue_msg;
mod link_eglise_departement_msg;

pub use search_msg::*;
pub use create_eglise_msg::*;
pub use create_departement_msg::*;
pub use create_langue_msg::*;
pub use link_eglise_departement_msg::*;

use common_crates::{
    serde::{self, Deserialize, Serialize}
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub enum AnnuaireMsg {
    GetInfosToCreateUser,
    GetInfosToLinkEgliseAndDepart,
    Search(AnnuaireSearchMsg),
    CreateEglises(Vec<AnnuaireCreateEgliseMsg>),
    CreateDepartements(Vec<AnnuaireCreateDepartementMsg>),
    CreateLangues(Vec<AnnuaireCreateLangueMsg>),
    LinkEgliseDepart(Vec<AnnuaireLinkEgliseDepartMsg>)
    
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
