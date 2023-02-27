use crate::structs::challenge::ChallengeValue;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct PublicLeaderBoard(pub Vec<PublicPlayer>);
#[derive(Serialize, Deserialize, Debug)]
pub struct PublicPlayer {
    pub name: String,
    stream_id: String,
    pub score: i32,
    steps: u32,
    pub is_active: bool,
    total_used_time: f64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RoundSummary {
    pub challenge: String,
    pub chain: Vec<ReportedChallengeResult>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BadResult {
    pub used_time: f64,
    pub next_target: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportedChallengeResult {
    pub name: String,
    pub value: ChallengeValue,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct EndOfGame {
    pub leader_board: PublicLeaderBoard,
}
