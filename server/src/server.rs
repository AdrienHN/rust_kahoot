//! `ChatServer` is an actor. It maintains list of connection client session.
//! And manages available rooms. Peers send messages to other peers in same
//! room through `ChatServer`.

use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use serde::Serialize;
use std::{
    collections::{HashMap, HashSet},
    sync::{atomic::AtomicUsize, Arc},
};

#[derive(Serialize)]
pub struct QuestionStruct {
    pub question_value: i32,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum SentMessage {
    Question(QuestionStruct),
}

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Message for chat server communications

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub code: String,
}
/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Start {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct UserResponse {
    pub id: usize,
    pub reponse: i32,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Clone, Debug)]
pub struct QuizzRoom {
    pub playerlist: HashSet<usize>,
    pub num_question: i32,
    pub answered_players: HashMap<usize, i32> // usize : id du joueur et i32 reponse du joueur
}

/// `ChatServer` manages chat rooms and responsible for coordinating chat session.
///
/// Implementation is very naïve.
//#[derive(Debug)]
pub struct ChatServer {
    sessions: HashMap<usize, Recipient<Message>>,
    //rooms: HashMap<String, HashSet<usize>>,
    rooms: HashMap<String, QuizzRoom>,
    rng: ThreadRng,
    //visitor_count: Arc<AtomicUsize>,
}

impl ChatServer {
    pub fn new(_visitor_count: Arc<AtomicUsize>) -> ChatServer {
        // default room
        let mut rooms = HashMap::new();
        let main_room = QuizzRoom::new();

        rooms.insert("Main".to_owned(), main_room);

        ChatServer {
            sessions: HashMap::new(),
            rooms,
            rng: rand::thread_rng(),
            //  visitor_count,
        }
    }

    /// Send message to all users in the room
    fn send_message(&self, room: &str, message: &str, skip_id: usize) {
        if let Some(tgdrtt) = self.rooms.get(room) {
            for id in &tgdrtt.playerlist {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(&id) {
                        let _ = addr.do_send(Message(message.to_owned()));
                    }
                }
            }
        }
    }
}

impl QuizzRoom {
    pub fn new() -> Self {
        QuizzRoom {
            playerlist: HashSet::new(),
            num_question: 0,
            answered_players: HashMap::new(),
        }
    }

    pub fn players_answered(&self) -> bool {
        for player_id in self.playerlist.iter() {
            if !self.answered_players.contains_key(player_id) {
                return false
            }
        }

        true
    }
}

/// Make actor from `ChatServer`
impl Actor for ChatServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");

        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);


        // auto join session to Main room
        self.rooms
            .entry("Main".to_owned())
            .or_insert(QuizzRoom::new())
            .playerlist
            .insert(id);

        // send id back
        id
    }
}
//Start message
impl Handler<Start> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Start, _: &mut Context<Self>) {
        println!("Start questions");

        let mut room = None;

        for (key, val) in self.rooms.iter() {
            if val.playerlist.contains(&msg.id) {
                room = Some(key);
                break;
            }
        }
        let msg = SentMessage::Question(QuestionStruct { question_value: 0 });
        if let Some(startsess) = self.rooms.get(room.unwrap()) {
            for id in &startsess.playerlist {
                if let Some(addr) = self.sessions.get(&id) {

                    let _ = addr.do_send(Message(serde_json::to_string(&msg).unwrap()));
                }
            }
        }
    }
}
// reponse utilisateur message
impl Handler<UserResponse> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: UserResponse, _: &mut Context<Self>) {
        println!("l'utilisateur à bien répondu {}", msg.reponse);
        let mut room : Option<String> = None;

        for (key, val) in self.rooms.iter() {
            if val.playerlist.contains(&msg.id) {
                room = Some(key.to_string());
                break;
            }
        }

        if let Some(quizzroom) = self.rooms.get_mut(&room.unwrap()) {
            quizzroom.answered_players.insert(msg.id, msg.reponse);
            if quizzroom.players_answered() == true {
                let msg = SentMessage::Question(QuestionStruct { question_value: 0 });
                for id in &quizzroom.playerlist {
                    if let Some(addr) = self.sessions.get(&id) {
                        let _ = addr.do_send(Message(serde_json::to_string(&msg).unwrap()));
                    }
                }
                            println!("Question suivante");
            }
        }
    }
}

// user join
impl Handler<Join> for ChatServer {
    type Result = ();

    fn handle(&mut self, _msg: Join, _: &mut Context<Self>) {
        println!("l'utilisateur à bien rejoin");
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnectedL");

        let mut rooms: Vec<String> = Vec::new();

        // remove address
        if self.sessions.remove(&msg.id).is_some() {
            // remove session from all rooms
            for (name, utildisc) in &mut self.rooms {
                if utildisc.playerlist.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }
        // send message to other users
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }
    }
}


