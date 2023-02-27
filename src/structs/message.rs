pub use crate::structs::connection::*;
pub use crate::structs::game::*;
pub use crate::structs::challenge::*;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub enum Message{
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(PublicLeaderBoard),
    Challenge(ChallengeFormat),
    ChallengeResult(ChallengeResult),
    ChallengeTimeout(ChallengeTimeout),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),
}