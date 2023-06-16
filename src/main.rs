use std::net::SocketAddr;

use obs_remote_1lt::websocket::Websocket;


fn main() -> Result<(), String>{
    
    let websocket = match Websocket::new(SocketAddr::from(([127, 0, 0, 1], 4455)), Some(format!("46Y6AzmrmHDoT5fz"))) {
        Ok(websocket) => websocket,
        Err(e) =>  { 
            println!("Error while connecting to OBS: {}", e);
            return Err(format!("Program shutdown because the connection to OBS Websocket could not be established"));
        }
    };

    
    
    Ok(())
}



//128 64 32 16 8 4 2 1