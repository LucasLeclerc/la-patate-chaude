use crate::structs::game::BadResult;
use crate::structs::md5::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeFormat{
    MD5HashCash(MD5HashCash),

}
#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeResult{
    answer:ChallengeAnswer,
    next_target:String,

}
#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeAnswer{
    MD5HashCash(MD5HashCashOutput),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeTimeout{
    message:String,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeValue{
    Unreachable,
    Timeout,
    BadResult(BadResult),
    Ok(Ok),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Ok {
    used_time: f64,
    next_target: String,
}