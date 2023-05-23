use serde_derive::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    pub last_name: String,
    pub first_name: String,
    pub password: String,
    pub user_token: String,
    pub email: String,
    pub two_factor: bool
}