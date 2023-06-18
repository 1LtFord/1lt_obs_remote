use std::net::SocketAddr;

use obs_remote_1lt::websocket::Websocket;


fn main() -> Result<(), String>{
    let rpcversion = 1;
    
    let mut websocket = match Websocket::new(SocketAddr::from(([127, 0, 0, 1], 4455)), Some(format!("46Y6AzmrmHDoT5fz")), rpcversion) {
        Ok(websocket) => websocket,
        Err(e) =>  { 
            println!("Error while connecting to OBS: {}", e);
            return Err(format!("Program shutdown because the connection to OBS Websocket could not be established"));
        }
    };

    obs_remote_1lt::toggle_scene_item(&mut websocket, "Game".to_string(), "Vorlage_Webcam".to_string())
}