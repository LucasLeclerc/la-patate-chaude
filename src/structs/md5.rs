use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCash{
    pub complexity:u32,
    pub message:String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashOutput{
    seed:u64,
    hashcode:String,
}