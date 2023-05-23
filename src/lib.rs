mod users;

use serde_derive::{Deserialize, Serialize};

pub use self::users::*;


#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    RegisterUser(RegisterUser)
}