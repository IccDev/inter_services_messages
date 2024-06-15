mod search_msg;
mod create_eglise_msg;
mod create_departement_msg;
mod create_langue_msg;
mod link_eglise_departement_msg;
mod create_certificat_msg;
mod create_competence_msg;
mod create_diplome_msg;
mod create_domaine_msg;
mod create_ecole_msg;
mod create_entreprise_msg;
mod create_plusinfos_msg;
mod create_specialite_msg;
mod create_titre_msg;
mod link_user_certificat_msg;
mod link_user_competence_msg;
mod link_user_contact_msg;
mod link_user_diplome_msg;
mod link_user_education_msg;
mod link_user_langue_msg;
mod link_user_pluseinfos_msg;
mod link_user_profession_msg;

pub use search_msg::*;
pub use create_eglise_msg::*;
pub use create_departement_msg::*;
pub use create_langue_msg::*;
pub use link_eglise_departement_msg::*;
pub use create_certificat_msg::*;
pub use create_competence_msg::*;
pub use create_diplome_msg::*;
pub use create_domaine_msg::*;
pub use create_ecole_msg::*;
pub use create_entreprise_msg::*;
pub use create_specialite_msg::*;
pub use create_plusinfos_msg::*;
pub use create_titre_msg::*;
pub use link_user_certificat_msg::*;
pub use link_user_competence_msg::*;
pub use link_user_contact_msg::*;
pub use link_user_diplome_msg::*;
pub use link_user_education_msg::*;
pub use link_user_langue_msg::*;
pub use link_user_pluseinfos_msg::*;
pub use link_user_profession_msg::*;
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
    CreateCertificats(Vec<AnnuaireCreateCertificatMsg>),
    CreateCompetences(Vec<AnnuaireCreateCompetenceMsg>),
    CreateDiplomes(Vec<AnnuaireCreateDiplomeMsg>),
    CreateDomaines(Vec<AnnuaireCreateDomaineMsg>),
    CreateEcoles(Vec<AnnuaireCreateEcoleMsg>),
    CreateEntreprises(Vec<AnnuaireCreateEntrepriseMsg>),
    CreatePlusInfos(Vec<AnnuaireCreatePlusInfosMsg>),
    CreateSpecialites(Vec<AnnuaireCreateSpecialiteMsg>),
    CreateTitres(Vec<AnnuaireCreateTitreMsg>),
    LinkEgliseDepart(Vec<AnnuaireLinkEgliseDepartMsg>),
    LinkUserCertificats(Vec<AnnuaireLinkUserCertificatMsg>),
    LinkUserCompetences(Vec<AnnuaireLinkUserCompetenceMsg>),
    LinkUserContacts(Vec<AnnuaireLinkUserContactMsg>),
    LinkUserDiplomes(Vec<AnnuaireLinkUserDiplomeMsg>),
    LinkUserEducations(Vec<AnnuaireLinkUserEducationMsg>),
    LinkUserLangues(Vec<AnnuaireLinkUserLangueMsg>),
    LinkUserPlusInfos(Vec<AnnuaireLinkUserPluseInfosMsg>),
    LinkUserProfessions(Vec<AnnuaireLinkUserProfessionMsg>)
}
