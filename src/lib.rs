use std::net::{TcpStream, SocketAddr};
use std::io::{prelude::*, BufReader};
use std::str;

use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};
use websocket::Websocket;

use crate::message::Message;
use crate::payload::Payload;

mod message;
mod header;
mod payload;
mod obs;
pub mod websocket;

pub struct ObsConnection {
    ip: SocketAddr,
    stream: TcpStream
}

pub fn toggle_scene_item(websocket: &mut Websocket, scene: String, scene_item: String) -> Result<(), String> {
    //get id
    let mut payload = "{".to_string();
    payload = payload + "\"op\":6,";
    payload = payload + "\"d\":{";
    payload = payload + "\"requestType\":\"GetSceneItemId\",";
    payload = payload + "\"requestId\":\"f819dcf0-89cc-11eb-8f0e-382c4ac93b9c\",";
    payload = payload + "\"requestData\":{";
    payload = format!("{payload}\"sceneName\":\"{scene}\",");
    payload = format!("{payload}\"sourceName\":\"{scene_item}\"");
    payload = payload + "}}}";
    println!("{payload}");

    let message = Message::new(true, crate::header::Opcode::TextFrame, true, payload);
    
    match websocket.send_message(message) {
        Ok(()) => (),
        Err(error) => return Err(error)
    }
    let message = match websocket.read_message() {
        Ok(message) => message,
        Err(error) => return Err(error)
    };
    let payload = match Payload::from_string(message.payload_value()) {
        Ok(payload) => payload,
        Err(error) => return Err(error)
    };

    let mut id_str = String::new();
    let mut id: usize = 0;
    for (key, value) in payload.attributes() {
        match key.as_str() {
            "sceneItemId" => {id_str = value},
            _ => ()
        }
    }
    if !id_str.is_empty() {
        id = match id_str.parse() {
            Ok(id) => id,
            Err(_) => return Err("Could not parse id of scene item (not a number)".to_string())
        }
    }
    else {
        return Err("id of scene item not found".to_string());
    }
    println!("{}", message.payload());

    //get enabled

    let mut payload = "{".to_string();
    payload = payload + "\"op\":6,";
    payload = payload + "\"d\":{";
    payload = payload + "\"requestType\":\"GetSceneItemEnabled\",";
    payload = payload + "\"requestId\":\"f819dcf0-89cc-11eb-8f0e-382c4ac93b9c\",";
    payload = payload + "\"requestData\":{";
    payload = format!("{payload}\"sceneName\":\"{scene}\",");
    payload = format!("{payload}\"sceneItemId\":{id}");
    payload = payload + "}}}";
    println!("{payload}");

    let message = Message::new(true, crate::header::Opcode::TextFrame, true, payload);
    
    match websocket.send_message(message) {
        Ok(()) => (),
        Err(error) => return Err(error)
    }
    let message = match websocket.read_message() {
        Ok(message) => message,
        Err(error) => return Err(error)
    };
    println!("{}", message.payload());

    //toggle
    payload = "{".to_string();
    payload = payload + "\"op\":6,";
    payload = payload + "\"d\":{";
    payload = payload + "\"requestType\":\"SetSceneItemEnabled\",";
    payload = payload + "\"requestId\":\"f819dcf0-89cc-11eb-8f0e-382c4ac93b9c\",";
    payload = payload + "\"requestData\":{";
    payload = format!("{payload}\"sceneName\":\"{scene}\",");
    payload = format!("{payload}\"sceneItemId\":{id},");
    if message.payload().contains("\"sceneItemEnabled\":true") {
        payload = payload + "\"sceneItemEnabled\":false";
    }
    else {
        payload = payload + "\"sceneItemEnabled\":true";
    }
    payload = payload + "}}}";
    println!("{payload}");

    let message = Message::new(true, crate::header::Opcode::TextFrame, true, payload);
    
    match websocket.send_message(message) {
        Ok(()) => (),
        Err(error) => return Err(error)
    }
    let message = match websocket.read_message() {
        Ok(message) => message,
        Err(error) => return Err(error)
    };
    println!("{}", message.payload());


    Ok(())
}