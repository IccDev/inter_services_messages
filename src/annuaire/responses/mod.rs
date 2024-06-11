mod search_response;
mod all_infos_to_create_user_response;
mod all_infos_to_link_eglise_and_depart_response;

pub use search_response::*;
pub use all_infos_to_create_user_response::*;
pub use all_infos_to_link_eglise_and_depart_response::*;

use common_crates::{
    serde::{self, Deserialize, Serialize}
};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "self::serde")]
pub enum AnnuaireResponse {
    Search(AnnuaireSearchRespone),
    Create(i32),
    Creates(Vec<i32>),
    AllInfosToCreateUser(AnnuaireAllInfosToCreateUserRespone),
    AllInfosToLinkEgliseAndDepart(AnnuaireInfosToLinkEgliseAndDepartRespone)
}

