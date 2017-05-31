//! Creates a ConnectK game server that listens on port 8080;
//! The server allows for client interaction using oData, and enables only
//! direct support for a few of the necessary functions for the service.
//! See the README for a detailed usage guide.
//!
//! A real system would likely use a database instead of global vector, but
//! it works for our purposes without going crazy on the scope creep. lazy_static!
//! is used for this purpose (see game_db.rs)


#[cfg(test)] 
extern crate hyper; 

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

use game_db::{GameServer, GAMES};
use connect_four::connect_four::{ConnectK, Player, GameStatus};


/// Declares Game, the EntityType for the ConnectK class. This will get serialized
/// by serde and sent as oData. 
defEntity!(Game(keys => id) {
    id: Int64,
    width: Int64,
    height: Int64,
    k: Int16,
    curr_player: Int16,
    status: String,
    board: String
});


/// Declares Games as an EntitySet containing entities of type Game. Once added to
/// the model, it will be reachable via <...>/Games(I) where I can be used to access
/// a game correponding to the id. These are stored in a vector of ConnectK.
defEntitySet!(Games, Game);


/// Convert the internal ConnectK representation to the serializable Game Entity type.
/// Major difference here is that enums are converted to either numbers/strings, and
/// the board is linerized and converted to a string.
fn convert(game: ConnectK, id: usize) -> Game
{
    // Convert game board to a string
    let board: String = game.board_linear().into_iter().map(|x| x.to_string()).collect();

    // Game::new is provided to us by defEntity!
    Game::new(
        id as i64,
        game.width as i64,
        game.height as i64,
        game.k as i16,

        match game.curr_player {
            Player::One => 1,
            Player::Two => 2
        },

        match game.status {
            GameStatus::InProcess => String::from("InProcess"),
            GameStatus::PlayerOneWin => String::from("PlayerOneWin"),
            GameStatus::PlayerTwoWin => String::from("PlayerTwoWin"),
            GameStatus::Tie => String::from("Tie")
        },

        board)
}


/// Here we implement the EntitySet trait for our Games EntitySet. We only directly implement
/// the functions we need. For mutating state, we provide an oData action "play_move" that
/// is tied to the model. 
impl EntitySet for Games {

    /// Create a new game when provided:
    ///    width:  Edm::Int64
    ///    height: Edm::Int64
    ///    k:      Edm::Int64
    fn create(&self, value: serde_json::Value) -> Res
    {
        let gamek = ConnectK::new(
            value["width"].as_i64().unwrap() as usize,
            value["height"].as_i64().unwrap() as usize,
            value["k"].as_i64().unwrap() as usize,

            match value["curr_player"].as_i64().unwrap(){
                1 => Player::One,
                2 => Player::Two,
                _ => panic!("Found unacceptable player number!")
            });

        // ids correspond to location in array
        let id = GameServer::insert(gamek.clone());
        Res::Created(json!(convert(gamek, id)))
    }

    /// Get a specific game with the designated key
    fn read(&self, key: String) -> Res
    {
        let id: i64 = key.parse().unwrap();
        match GameServer::select(id) {
            Some(game) => Res::Succ(Some(json!(convert(game, id as usize)))),
            None       => Res::Err(Error::NotFound(String::from("Games")))
        }
    }

    /// Get a list of all available games
    fn read_list(&self) -> Res
    {
        let mut id: i64 = -1;
        let mut id = || -> i64 {id += 1; id};
        
        let games = GameServer::select_all();
        let ogames: Vec<Game> = games.into_iter()
            .map(|game| convert(game, id() as usize))
            .collect();
        
        Res::Succ(Some(json!(ogames)))
    }
}


fn run()
{

    // Declare a new model, adding our previously defined Games EntitySet
    let model = ModelBuilder::new("connect_four.svc")
        .add(Games::declare())

        // Defines an action "play_move". OData actions are basically call that
        // can be used to mutate state. play_move calls the internal ".insert"
        // method of ConnectK
        .action("play_move", vec![edm::Type::String], |v: serde_json::Value| -> Res {

            let id = v["id"].as_u64().expect("Could not parse id");
            let player_move = v["move"].as_u64().expect("Could not parse move");
            
            match (*GAMES.lock().expect("Could not unwrap")).get_mut(id as usize) {
                Some(game) => {
                    match game.insert(player_move as usize) {
                        Ok(_) => Res::Succ(None),
                        _     => Res::Err(Error::InvalidParameter)
                    }
                    
                },
                None => Res::Err(Error::NotFound(String::from("Games")))
            }
        })
        .build();

    // Declare a new service that uses our model. We'll now be able to say, access the first
    // game, by making a GET request to <server:port>/api/connect_four.svc/Games(0)
    let svc = ServiceBuilder::new("api")
        .add(model)
        .build();

    svc.start();
}


fn main()
{
    run();
}


#[cfg(test)]
mod test {
    
    use super::*;
    use std::thread;
    use std::time::Duration;
    use hyper::Client;
    use hyper::status::StatusCode;
    use serde_json::{Value, from_reader};
    
    #[test]
    fn check_service()
    {
        thread::spawn(move || { run(); });

        // Technically a race ...but we are lazy so let's assume 200 msec is
        // enough for server to startup
        thread::sleep(Duration::from_millis(200));

        { // Test read list
            println!("Checking Read List");
            let client = Client::new();
            let url = "http://localhost:8080/api/connect_four.svc/Games";
            let res = client.get(url).send().unwrap();
            assert_eq!(res.status, StatusCode::Ok);
        }

        { // Test create
            println!("Checking Create");
            let client = Client::new();
            let url = "http://localhost:8080/api/connect_four.svc/Games";
            let value = json!({
                "curr_player": 1,
                "height": 7,
                "width": 5,
                "k": 4
            });

            //The response includes the id of the game we created
            let res = client.post(url).body(&value.to_string()).send().unwrap();
            assert_eq!(res.status, StatusCode::Created);

            //Parse response body
            let data: Value = from_reader(res).expect("Unable to parse response!");
            let id = data["id"].as_u64().expect("Unable to parse id!");
            
            //We should now be able to get the item we created
            println!("Checking Read Created");
            let url = &format!("http://localhost:8080/api/connect_four.svc/Games({})", id);
            let res = client.get(url).send().unwrap();
            assert_eq!(res.status, StatusCode::Ok);

            // Lets now play a move
            let url = "http://localhost:8080/api/connect_four.svc/play_move";
            let value = json!({
                "id": id,
                "move": 0
            });
            println!("{}", &value.to_string());
            client.post(url).body(&value.to_string()).send().unwrap();

            // Check that the move was actually played
            println!("Checking Move Played");
            let url = &format!("http://localhost:8080/api/connect_four.svc/Games({})", id);
            println!("{}", url);
            let res = client.get(url).send().unwrap();
            assert_eq!(res.status, StatusCode::Ok);

            let data: Value = from_reader(res).expect("Unable to parse response!");
            let board = data["board"].as_str().expect("Unable to parse id!");
            assert_eq!(board.chars().nth(0).unwrap(), '1');
        }

    }
}





