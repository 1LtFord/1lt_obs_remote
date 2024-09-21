use websocket::Websocket;

use crate::message::Message;
use crate::payload::Payload;

mod message;
mod header;
mod payload;
mod obs;
pub mod websocket;

pub fn toggle_scene_item(websocket: &mut Websocket, scene: &String, scene_item: &String) -> Result<(), String> {
    //get id
    let id = match scene_item_id(websocket, &scene, &scene_item) {
        Ok(id) => id,
        Err(error) => return Err(error)
    };

    //toggle
    let show = match scene_item_enabled(websocket, &scene, &id) {
        Ok(enabled) => !enabled, 
        Err(error) => return Err(error)
    };
    set_scene_item_show_status(websocket, &scene, &id, show)
}

pub fn set_scene_item_status(websocket: &mut Websocket, scene: &String, scene_item: &String, show: bool) -> Result<(), String> {
    //get id
    let id = match scene_item_id(websocket, &scene, &scene_item) {
        Ok(id) => id,
        Err(error) => return Err(error)
    };

    //get current scene item status
    let enabled = match scene_item_enabled(websocket, &scene, &id) {
        Ok(enabled) => enabled, 
        Err(error) => return Err(error)
    };

    if enabled != show {
        set_scene_item_show_status(websocket, &scene, &id, show)
    }
    else {
        Ok(())
    }
}

pub fn show_scene(websocket: &mut Websocket, scene: &String) -> Result<(), String> {
    let mut payload = "{".to_string();
    payload = payload + "\"op\":6,";
    payload = payload + "\"d\":{";
    payload = payload + "\"requestType\":\"SetCurrentProgramScene\",";
    payload = payload + "\"requestId\":\"f819dcf0-89cc-11eb-8f0e-382c4ac93b9c\",";
    payload = payload + "\"requestData\":{";
    payload = format!("{payload}\"sceneName\":\"{scene}\"");
    payload = payload + "}}}";
    println!("{payload}");

    println!("{payload}");

    let _answer = match send_and_recieve(websocket, payload) {
        Ok(answer) => answer,
        Err(error) => return Err(error)
    };

    Ok(())
}

fn set_scene_item_show_status(websocket: &mut Websocket, scene: &String, id: &usize, show: bool) -> Result<(), String> {
    let mut payload = "{".to_string();
    payload = payload + "\"op\":6,";
    payload = payload + "\"d\":{";
    payload = payload + "\"requestType\":\"SetSceneItemEnabled\",";
    payload = payload + "\"requestId\":\"f819dcf0-89cc-11eb-8f0e-382c4ac93b9c\",";
    payload = payload + "\"requestData\":{";
    payload = format!("{payload}\"sceneName\":\"{scene}\",");
    payload = format!("{payload}\"sceneItemId\":{id},");
    if show {
        payload = payload + "\"sceneItemEnabled\":true";
    }
    else {
        payload = payload + "\"sceneItemEnabled\":false";
    }
    payload = payload + "}}}";
    println!("{payload}");

    let _answer = match send_and_recieve(websocket, payload) {
        Ok(answer) => answer,
        Err(error) => return Err(error)
    };

    Ok(())
}

fn scene_item_enabled(websocket: &mut Websocket, scene: &String, id: &usize) -> Result<bool, String> {
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
    
    let answer = match send_and_recieve(websocket, payload) {
        Ok(answer) => answer,
        Err(error) => return Err(error)
    };
    
    if answer.payload().contains("\"sceneItemEnabled\":") {
        Ok(answer.payload().contains("\"sceneItemEnabled\":true"))
    }
    else {
        Err("Could not get status of scene item. Make sure it exists".to_string())
    }
    
}

fn scene_item_id(websocket: &mut Websocket, scene: &String, scene_item: &String) -> Result<usize, String> {
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

    let answer = match send_and_recieve(websocket, payload) {
        Ok(answer) => answer,
        Err(error) => return Err(error)
    };
    let payload = match Payload::from_string(answer.payload_value()) {
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
    Ok(id)
}

pub fn create_record_chapter(websocket: &mut Websocket) -> Result<(), String> {
    let mut payload = "{".to_string();
    payload = payload + "\"op\":6,";
    payload = payload + "\"d\":{";
    payload = payload + "\"requestType\":\"CreateRecordChapter\",";
    payload = payload + "\"requestId\":\"f819dcf0-89cc-11eb-8f0e-382c4ac93b9c\",";
    payload = payload + "\"requestData\":{";
    payload = format!("{payload}\"chapterName\":\"marker\"");
    payload = payload + "}}}";
    println!("{payload}");

    let _answer = match send_and_recieve(websocket, payload) {
        Ok(answer) => answer,
        Err(error) => return Err(error)
    };

    Ok(())
}

fn send_and_recieve(websocket: &mut Websocket, payload: String) -> Result<Message, String>{
    let message = Message::new(true, crate::header::Opcode::TextFrame, true, payload);
    
    match websocket.send_message(message) {
        Ok(()) => (),
        Err(error) => return Err(error)
    }
    let answer = match websocket.read_message() {
        Ok(answer) => answer,
        Err(error) => return Err(error)
    };
    println!("{}", answer.payload());

    Ok(answer)
}