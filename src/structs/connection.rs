use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Welcome{
    pub version: i8,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe{
    name: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeResult{
    Ok,
    Err(SubscribeError),
}
#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName,
}