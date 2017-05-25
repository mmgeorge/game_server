
use std::sync::Mutex;
use connect_four::connect_four::{ConnectK, Player};


/// Emulate a database server
lazy_static! {
    // Vector index = key
    static ref GAMES: Mutex<Vec<ConnectK>> = {
        let mut vec = Vec::new();
        vec.push(ConnectK::new(5, 7, 4, Player::One));

        Mutex::new(vec)
    };
}

/// Encapsolates emulated database server
pub struct GameServer {}

impl GameServer {
    pub fn select<'a> (key: i64) -> Option<ConnectK>
    {
        let guard = GAMES.lock().unwrap();
        match (*guard).get(key as usize) {
            Some(game) => Some(game.clone()),
            None       => None
        }
    }

    pub fn select_all () -> Vec<ConnectK>
    {
        (*GAMES.lock().unwrap()).clone()
    }


    pub fn insert (game: ConnectK) -> usize
    {
        let ref mut games = *GAMES.lock().unwrap();
        games.push(game);
        games.len()
    }
}




