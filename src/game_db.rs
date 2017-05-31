//! This module basically defines a wrapper around the global vector
//! which is used to store active ConnectK games. A real game server
//! would want to use a database for performance and persistence. 

use std::sync::Mutex;
use connect_four::connect_four::{ConnectK, Player};


/// Defines a global vector of ConnectK instances. Used to
/// emulate a database server
lazy_static! {
    // Vector index = key
    pub static ref GAMES: Mutex<Vec<ConnectK>> = {
        let mut vec = Vec::new();
        vec.push(ConnectK::new(5, 7, 4, Player::One));

        Mutex::new(vec)
    };
}


/// Acts to encapsolates access to the emulated database server
pub struct GameServer {}

impl GameServer {

    /// Select the game with the given key
    pub fn select<'a> (key: i64) -> Option<ConnectK>
    {
        let guard = GAMES.lock().unwrap();
        match (*guard).get(key as usize) {
            Some(game) => Some(game.clone()),
            None       => None
        }
    }

    /// Select all games present in the datbase
    pub fn select_all () -> Vec<ConnectK>
    {
        (*GAMES.lock().unwrap()).clone()
    }
    
    /// Add a game to the databse
    pub fn insert (game: ConnectK) -> usize
    {
        let ref mut games = *GAMES.lock().unwrap();
        games.push(game);
        games.len() - 1
    }
}




