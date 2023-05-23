mod users;

use serde_derive::{Deserialize, Serialize};
use remoc::rch;

pub use self::users::*;


#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub data: MessageData,
    pub sender: rch::oneshot::Sender<Result<(), String>>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageData {
    RegisterUser(RegisterUser)
}