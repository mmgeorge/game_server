# game_server
Server executable program for ConnectFour project. The server communicates with clients via oData. See examples below.

## Examples
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
