mod structs;
mod challenge2;

use std::str;
use std::net::TcpStream;
use std::io::{self,prelude::*,BufReader,Write};
use serde::{Serialize, Deserialize};
use std::time::{Duration, Instant};
use crate::structs::message::*;

use crate::challenge2::base::*;
use crate::challenge2::md5Challenge::*;




fn main() {
    println!("Entrer votre nom:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed To read Input");
    let mut server = "localhost:7878".to_string();
    let mut stream = TcpStream::connect(server).unwrap();
    if setup(&mut stream,&mut name) ==false {return ;};
    while round(&mut stream,&mut name)==true{

    }
    return;
    
    
}

fn round(stream: &mut TcpStream,player:&mut str)->bool{    
    let mut remainPlayers:Vec<String> = Vec::new();
    let mut finish=false;
    while !finish{
        let servMessage=readMessage(stream);
        match servMessage{
            Ok(Message::PublicLeaderBoard(list))=>{
                println!("Board :");
                showBoard(&list);
                getAvailablePlayer(&list,&mut remainPlayers);
            },
            Ok(Message::Challenge(challenge))=>{
                match challenge{
                    ChallengeFormat::MD5HashCash(data) => {
                        let starti = Instant::now();
                        let chall:md5Chall=Challenge::new(StructInput { complexity: (data.complexity), message: (data.message) });
                        let answer=chall.solve();
                        let duration = starti.elapsed();
                        let response=format!("{{\"ChallengeResult\":{{\"answer\":{{\"MD5HashCash\":{{\"seed\":{},\"hashcode\":\"{}\"}}}},\"next_target\":\"oui\"}}}}",answer.seed,answer.hashcode);
                        println!("{}",response);
                        sendMessage(&response, stream);
                        println!("Time elapsed in expensive_function() is: {:?}", duration);
                    }
                }
                println!("Challenge");
            },
            Ok(Message::ChallengeTimeout(ChallengeTimeout))=>{
                println!("Sorry you lost");
                return false
            },
            Ok(Message::RoundSummary(summary))=>{
                println!("summary");
                showSummary(&summary);
                finish=true;
                return true;
            },
            Ok(Message::EndOfGame(EndOfGame{ leader_board }))=>{
                println!("End of the game :");
                showBoard(&leader_board);
                return false;
            },
            _=>return false,
        }
        for i in &remainPlayers {
            println!("{}", i);
        }
    }
    return false;
}

fn showBoard(list:&PublicLeaderBoard){
    for i in &list.0{
        println!("player:{}, score:{}, is alive:{}", i.name,i.score,i.is_active);
    }
}
fn showSummary(summary:&RoundSummary){
    
}
fn getAvailablePlayer(list:&PublicLeaderBoard, remain:&mut Vec<String>){
    for i in &list.0{
        if(i.is_active){
            remain.push(i.name.to_owned());
        }
    }
}

fn setup(stream: &mut TcpStream,name :&mut String)->bool{
    if connectToServer(stream) == false {
        return false;
    }
    println!("Inscription en cours . . .");
    let message =format!("{{\"Subscribe\":{{\"name\":\"{}\"}}}}", name.trim());
    sendMessage(message.as_str(),stream);
    let subscribe=readMessage(stream);
    match subscribe{
        Ok(Message::SubscribeResult(result))=>{
            match result{
                SubscribeResult::Ok=>{
                    println!("Inscription reussi !!!");
                    return true;
                },
                SubscribeResult::Err(error)=>{
                    println!("Inscription impossible : {:?} !!!",error);
                    return false;
                }
            }
        }
        _=>return false,
    }
    return false;
}

fn verifSubscribe(message: String) -> bool {
    let startPos=match message.find("\":\""){
        Some(n) => n+3,
        None => {
            println!("None left");
            0
        }
    };
    let endPos=match message[startPos .. ].find("\"}"){
        Some(n) => n,
        None => {
            println!("None left");
            0
        }
    };
    let result=&message[startPos .. ][ .. endPos];
    if result == "Ok" {
        println!("Inscription reussi !!!");
        return true;
    };
    if result == "AlreadyRegistered" {
        println!("Le joueur existe deja :( ");
        return false;
    }
    if result == "InvalidName" {
        println!("Le nom du joueur n'est as valide :( ");
        return false;
    }
    return false;
    

}

fn connectToServer(stream: &mut TcpStream)->bool{
    println!("Connection au serveur . . .");
    sendMessage("\"Hello\"", stream);
    let message=readMessage(stream);
    match message{
        Ok(Message::Welcome(version))=>{
            println!("Connection au serveur version:{} reussi !!!",version.version);
            return true;
        }
        _=>{            
            println!("Erreur impossible de se connecter au serveur !");
            return false;
        }
    }
}
/*
fn getServMessage<'a>( stream: &'a mut TcpStream,message:&'a str)->Vec<&String>{
    sendMessage(message, stream);
    let read:&str=&readMessage(stream)[..];
    let param=getFirstParam(read);
    let word:Vec<&String>= param.into_iter().map(|s| s.to_string()).collect();
    println!("Message read : {}",read);
    return word;
}*/

fn sendMessage(message: &str, stream: &mut TcpStream) {
    let len = message.len() as u32;
    stream.write(&len.to_be_bytes()).unwrap();
    stream.write(message.as_bytes()).unwrap();    
}/*
fn getFirstParam<'a>(mut message: &'a str)->&'a str{
    message=&message[2.. ];
    let endPos=match message.find("\":"){
        Some(n) => n,
        None => {
            println!("None left");
            0
        }
    };
    let res =&message[.. endPos];
    return res;

} */

fn readMessage(stream: &mut TcpStream) -> Result<Message,serde_json::Error>{    
    println!("Attente de message du serveur . . .");
    let mut buf_len = [0u8; 4];
    stream.read_exact(buf_len.as_mut()).unwrap();

    let len = u32::from_be_bytes(buf_len);
    let mut buf = vec![0u8; len as usize];
   stream.read(&mut buf) ;
            // Convertir le message en chaîne de caractères et l'afficher
            let message = &String::from_utf8_lossy(&buf);
            let welcome_serialized = serde_json::to_string(&message).unwrap();
            let a = welcome_serialized.replace("\\", "");

            let first_last_off: &str = &a[1..a.len() - 1];
            let message2: Result<Message,_> = serde_json::from_str(&first_last_off);

            match message2 {
                Ok(ref m) => println!("message={m:#?}"),
                Err(ref err) => println!("error={err:?}")
            }
            //println!("Message reçu du serveur: {} / {} / {}", message,buf.len(),&message[0..5]);
            return message2;
        
    
}


/*
LOGS ERREURS :
invalid type: unit variant, expected newtype variant
message: Illegal message: Welcome(Welcome { version: 1 })
unknown variant `version`, expected one of `Hello`, `Welcome`, `StartGame`, `Subscribe`, `SubscribeResult`, `PublicLeaderBoard`, `Challenge`, `ChallengeResult`, `ChallengeTimeout`, `RoundSummary`, `EndOfGame` at line 1 column 10
Illegal message: Subscribe(Subscribe { name: "free_patato" })




    let len = message.len() as u32;

    stream.write(&len.to_be_bytes()).unwrap();
    stream.write(message.as_bytes()).unwrap();

    let mut buf_len = [0u8; 4];
    stream.read_exact(buf_len.as_mut()).unwrap();

    let len = u32::from_be_bytes(buf_len);
    let mut buf = vec![0u8; len as usize];
    stream.read_exact(buf.as_mut()).unwrap();

    println!("{buf:?}"); 
    let s = String::from_utf8_lossy(&buf);
    println!("{s}");
*/