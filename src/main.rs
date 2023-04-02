use std::net::SocketAddr;

use obs_remote_1lt::ObsConnection;


fn main() -> Result<(), String>{
    
    let obs = match ObsConnection::new(SocketAddr::from(([127, 0, 0, 1], 4455))) {
        Ok(obs) => obs,
        Err(e) =>  { 
            println!("Error while connecting to OBS: {}", e);
            return Err(format!("Program shutdown because the connection to OBS Websocket could not be established"));
        }
    };

    Ok(())
}



//128 64 32 16 8 4 2 1