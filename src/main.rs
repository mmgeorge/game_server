//! Creates a game server that listens on port 8080;
//! GET  localhost:8080/api/connect_four.svc/$metadata ->  metadata
//! GET  localhost:8080/api/connect_four.svc/Games     ->  list of games
//! GET  localhost:8080/api/connect_four.svc/Games(0)  ->  game with id=0
//! POST localhost:8080/api/connect_four.svc/Games     ->  create a new game
//!      with body = {"curr_player": 1, "height": 7, "id": 4, "k": 4,"width": 5 }


extern crate connect_four;
#[macro_use] extern crate odata;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate lazy_static;

mod game_db;

use odata::*;
use odata::model::ModelBuilder;
use odata::service::{ServiceBuilder, Res, Error}; 
use odata::entity::*;

use game_db::GameServer;
use connect_four::connect_four::{ConnectK, Player};


/// The entiy type for board
defEntity!(Game(keys => id) {
    id: Int64,
    width: Int64,
    height: Int64,
    k: Int16,
    curr_player: Int16,
    board: String
});

defEntitySet!(Games, Game);


/// Converts internal ConnectK representation to oData model
fn convert(game: ConnectK, id: usize) -> Game
{
    let board: String = game.board_linear().into_iter().map(|x| x.to_string()).collect();
    Game::new(id as i64, game.width as i64, game.height as i64, game.k as i16,
              match game.curr_player { Player::One => 1, Player::Two => 2 },
              board)
}


impl EntitySet for Games {

    fn create(&self, value: serde_json::Value) -> Res
    {
        let gamek = ConnectK::new(value["width"].as_i64().unwrap() as usize,
                                  value["height"].as_i64().unwrap() as usize,
                                  value["k"].as_i64().unwrap() as usize,
                                  match value["curr_player"].as_i64().unwrap(){
                                      1 => Player::One,
                                      2 => Player::Two,
                                      _ => panic!("Found unacceptable player number!")
                                  });
        //let game = convert(gamek.clone());
        let id = GameServer::insert(gamek.clone());
        Res::Succ(Some(json!(convert(gamek, id))))
    }
    
    fn read(&self, key: String) -> Res
    {
        let id: i64 = key.parse().unwrap();
        match GameServer::select(id) {
            Some(game) => Res::Succ(Some(json!(convert(game, id as usize)))),
            None       => Res::Err(Error::NotFound(String::from("Games")))
        }
    }
    
    fn read_list(&self) -> Res
    {
        let mut id: i64 = -1;
        let mut id = || -> i64 {id += 1; id};
        
        let games = GameServer::select_all();
        let ogames: Vec<Game> = games.into_iter().map(|game| convert(game, id() as usize)).collect();
        
        Res::Succ(Some(json!(ogames)))
    }
}


fn main() {
    let model = ModelBuilder::new("connect_four.svc")
        .add(Games::declare())
        .build();

    let svc = ServiceBuilder::new("api")
        .add(model)
        .build();

    svc.start();
}


