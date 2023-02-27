mod chall;
mod structs;

use crate::structs::message::*;
use std::env;
use std::io::{ prelude::*, Write};
use std::net::TcpStream;
use std::str;
use std::time::Instant;

use crate::chall::base::*;
use crate::chall::md5_challenge::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let mut name = "Groupe-4-4AL1".to_owned();
    let server = format!("{}:7878", args[1]).to_string();
    let mut stream = TcpStream::connect(server).unwrap();
    if setup(&mut stream, &mut name) == false {
        return;
    };
    while round(&mut stream, &mut name) == true {}
    return;
}

fn round(stream: &mut TcpStream, player: &mut str) -> bool {
    let mut remain_players: Vec<String> = Vec::new();
    let finish = false;
    while !finish {
        let serv_message = read_message(stream);
        match serv_message {
            Ok(Message::PublicLeaderBoard(list)) => {
                println!("Board :");
                show_board(&list);
                get_available_player(&list, &mut remain_players);
            }
            Ok(Message::Challenge(challenge)) => match challenge {
                ChallengeFormat::MD5HashCash(data) => {
                    let starti = Instant::now();
                    let chall: Md5chall = Challenge::new(StructInput {
                        complexity: (data.complexity),
                        message: (data.message),
                    });
                    let answer = chall.solve();
                    let duration = starti.elapsed();

                    let response=format!("{{\"ChallengeResult\":{{\"answer\":{{\"{}\":{{\"seed\":{},\"hashcode\":\"{}\"}}}},\"next_target\":\"{}\"}}}}",Md5chall::name(),answer.seed,answer.hashcode,get_target(&mut remain_players,player));
                    println!("{}", response);
                    send_message(&response, stream);
                    println!("Time elapsed in expensive_function() is: {:?}", duration);
                }
            },
            Ok(Message::ChallengeTimeout(challenge_timeout)) => {
                println!("Sorry you lost \n {}", challenge_timeout.message);
                return false;
            }
            Ok(Message::RoundSummary(summary)) => {
                println!("Round summary :");
                show_summary(&summary);
                return true;
            }
            Ok(Message::EndOfGame(EndOfGame { leader_board })) => {
                println!("End of the game :");
                show_board(&leader_board);
                return false;
            }
            _ => return false,
        }
    }
    return false;
}

fn show_board(list: &PublicLeaderBoard) {
    for i in &list.0 {
        println!(
            "player:{}, score:{}, is alive:{}",
            i.name, i.score, i.is_active
        );
    }
}

fn show_summary(summary: &RoundSummary) {
    println!("Game played : {}", summary.challenge);
    for i in &summary.chain {
        match &i.value {
            ChallengeValue::Unreachable => {
                println!("Player {} is unreachable !!!", i.name);
            }
            ChallengeValue::Timeout => {
                println!("Player {} get a timeout !!!", i.name);
            }
            ChallengeValue::BadResult(bad_result) => {
                println!(
                    "Player {} get a bad result: take {}, next target {}",
                    i.name, bad_result.used_time, bad_result.next_target
                );
            }
            ChallengeValue::Ok(ok) => {
                println!(
                    "Player {} answering well: take {}, next target {}",
                    i.name, ok.used_time, ok.next_target
                );
            }
        }
    }
}

fn get_available_player(list: &PublicLeaderBoard, remain: &mut Vec<String>) {
    for i in &list.0 {
        if i.is_active {
            remain.push(i.name.to_owned());
        }
    }
}

fn get_target(remain: &mut Vec<String>, player: &mut str) -> String {
    println!("player : {}", player);
    for i in remain {
        if i != &player.to_string() {
            return i.to_owned();
        }
    }
    return "".to_owned();
}

fn setup(stream: &mut TcpStream, name: &mut String) -> bool {
    if connect_to_server(stream) == false {
        return false;
    }
    println!("Inscription en cours . . .");
    let message = format!("{{\"Subscribe\":{{\"name\":\"{}\"}}}}", name.trim());
    send_message(message.as_str(), stream);
    let subscribe = read_message(stream);
    match subscribe {
        Ok(Message::SubscribeResult(result)) => match result {
            SubscribeResult::Ok => {
                println!("Inscription reussi !!!");
                return true;
            }
            SubscribeResult::Err(error) => {
                println!("Inscription impossible : {:?} !!!", error);
                return false;
            }
        },
        _ => return false,
    }
}

fn connect_to_server(stream: &mut TcpStream) -> bool {
    println!("Connection au serveur . . .");
    send_message("\"Hello\"", stream);
    let message = read_message(stream);
    match message {
        Ok(Message::Welcome(version)) => {
            println!(
                "Connection au serveur version:{} reussi !!!",
                version.version
            );
            return true;
        }
        _ => {
            println!("Erreur impossible de se connecter au serveur !");
            return false;
        }
    }
}

fn send_message(message: &str, stream: &mut TcpStream) {
    let len = message.len() as u32;
    stream.write(&len.to_be_bytes()).unwrap();
    stream.write(message.as_bytes()).unwrap();
}

fn read_message(stream: &mut TcpStream) -> Result<Message, serde_json::Error> {
    println!("Attente de message du serveur . . .");
    let mut buf_len = [0u8; 4];
    stream.read_exact(buf_len.as_mut()).unwrap();

    let len = u32::from_be_bytes(buf_len);
    let mut buf = vec![0u8; len as usize];
    let res=stream.read(&mut buf);
    match res{
        Ok(ref _value)=>{
            let message = &String::from_utf8_lossy(&buf);
            let welcome_serialized = serde_json::to_string(&message).unwrap();
            let a = welcome_serialized.replace("\\", "");
        
            let first_last_off: &str = &a[1..a.len() - 1];
            let message2: Result<Message, _> = serde_json::from_str(&first_last_off);
        
            match message2 {
                Ok(ref m) => println!("message={m:#?}"),
                Err(ref err) => println!("error={err:?}"),
            }
            return message2;
        },
        Err(ref err)=>{
            println!("error={err:?}");
            return serde_json::from_str(&err.to_string().to_owned());
        }
    }
}
