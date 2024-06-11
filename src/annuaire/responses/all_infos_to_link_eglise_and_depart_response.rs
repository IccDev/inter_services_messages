use common_crates::{
    serde::{self, Deserialize, Serialize}
};
use crate::annuaire::model::{Eglise, Departement};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub struct AnnuaireInfosToLinkEgliseAndDepartRespone {
    pub eglises: Vec<Eglise>,
    pub departements: Vec<Departement>
}

