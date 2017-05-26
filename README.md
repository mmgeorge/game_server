# game_server
Server executable program for ConnectFour project. The server communicates with clients via oData. See examples below.

## Quickstart Client Example
Let's create a quick simple client program that checks the status of a particular game. First add the following dependencies:
#### Cargo.toml
```
[dependencies]           
hyper = "0.10"           
serde_json = "1.0"       
serde_derive = "1.0.4"   
serde = "1.0.4"          
```
Now let's add in our dependencies. serde is used for json serialization/deserialization, and hyper will allow us to easily start up a client. 
#### main.rs
```
#[macro_use] extern crate serde_json;  
extern crate hyper;                    
                                       
use hyper::Client;                     
use hyper::status::StatusCode;         
use serde_json::{Value, from_reader};  
```
Then we write a quick main() function that does a GET and returns the resulting json. 
#### main.rs
```
fn main() {                                                                       
    let client = Client::new();                                                   
                                                                                  
    // Assuming we are hosting the server locally:                                
    let url = "http://localhost:8080/api/connect_four.svc/Games(0)";              
    let response = client.get(url).send().unwrap();                               
    assert_eq!(response.status, StatusCode::Ok); // sanity check                  
                                                                                  
    // Parse JSON                                                                 
    let value: Value = from_reader(response).expect("Unable to parse response!"); 
    println!("Body: {:?}", value);                                                
    println!("Got Board: id = {}, height = {}, width = {}",                       
             value["id"], value["height"], value["width"]);                       
}                                                                                 
```
The example can be found [here](https://github.com/mmgeorge/game_client_example).
##### Additional Reading: 
- [serde_json](https://docs.serde.rs/serde_json/) docs
- [hyper](https://hyper.rs/hyper/v0.10.9/hyper/client/struct.Client.html) client docs

## REST Examples
#### Query metadata document

`GET  <hostname>:8080/api/connect_four.svc/$metadata`

##### Response Payload:
```
{
    "$schema": "http://docs.oasis-open.org/odata/odata-json-csdl/v4.0/edm.json#",
    "actions": "",
    "definitions": {
        "Game": {
            "keys": [
                "id"
            ],
            "properties": {
                "board": {
                    "format": "",
                    "type": [
                        "string"
                    ]
    ...
    "entityContainer": {
        "entitySets": {
            "Games": {
                "entityType": {
                    "$ref": "#/definitions/Game"
                }
            }
        },
        "name": "ServiceName"
    },
    "functions": "",
    "odata-version": "4.0",
    "references": "",
    "schemas": "",
    "terms": ""
}
```
---

#### Get a list of games. 
`GET  <hostname>:8080/api/connect_four.svc/Games`

##### Response Payload:
```
[
  {
    "board": "00000000000000000000000000000000000",
    "curr_player": 1,
    "height": 7,
    "id": 0,
    "k": 4,
    "width": 5
  },
  {
    "board": "00000000000000000000000000000000000",
    "curr_player": 1,
    "height": 7,
    "id": 1,
    "k": 4,
    "width": 5
  },
  {
    "board": "00000000000000000000000000000000000",
    "curr_player": 1,
    "height": 7,
    "id": 2,
    "k": 4,
    "width": 5
  },
]
```
---

#### Get a specific game with id 0
`GET  <hostname>:8080/api/connect_four.svc/Games(0)`

##### Response Payload:
```
{
  "board": "00000000000000000000000000000000000",
  "curr_player": 1,
  "height": 7,
  "id": 0,
  "k": 4,
  "width": 5
}
```
---

#### Create a new game
`POST  <hostname>:8080/api/connect_four.svc/Games`
##### Request Body:
```
{
  "curr_player": 1,
  "height": 7,
  "k": 4,
  "width": 5
}
```

##### Response Payload:
```
{
  "board": "00000000000000000000000000000000000",
  "curr_player": 1,
  "height": 7,
  "id": 1,
  "k": 4,
  "width": 5
}
```
