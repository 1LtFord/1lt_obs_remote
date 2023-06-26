use std::env;
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

    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    if args.len() > 1 {
        match args[1].as_str() {
            "toggle" => toggle_scene_item(websocket, args),
            "show_one" => show_one_scene_item(websocket, args),
            "show_scene" => show_scene(websocket, args),
            _ => Err("Argument not recognised".to_string())
        }
    }
    else {
        Ok(())
    }
}

fn toggle_scene_item(mut websocket: Websocket, args: Vec<String>) -> Result<(), String> {
    if args.len() >= 4 {
        obs_remote_1lt::toggle_scene_item(&mut websocket, &args[2], &args[3])
    }
    else {
        if args.len() == 2 {
            return Err("Missing arguments scene and scene item to toggle".to_string())
        }
        else {
            return Err("Missing argument scene item to toggle".to_string())
        }   
    }
}

fn show_scene(mut websocket: Websocket, args: Vec<String>) -> Result<(), String> {
    if args.len() >= 3 {
        obs_remote_1lt::show_scene(&mut websocket, &args[2])
    }
    else {
        return Err("Missing argument scene to show".to_string())
    }
}

fn show_one_scene_item(mut websocket: Websocket, args: Vec<String>) -> Result<(), String> {
    if args.len() >= 5 {
        match obs_remote_1lt::set_scene_item_status(&mut websocket, &args[2], &args[3], true) {
            Ok(()) => (),
            Err(error) => return Err(error)
        }

        for scene_item in &args[4..] {
            match obs_remote_1lt::set_scene_item_status(&mut websocket, &args[2], scene_item, false) {
                Ok(()) => (),
                Err(error) => return Err(error)
            }
        }
    }
    else {
        if args.len() == 2 {
            return Err("Missing arguments scene, scene item to toggle and scene item to hide".to_string())
        }
        else if args.len() == 3 {
            return Err("Missing argument scene item to show and scene item to hide".to_string())
        }
        else {
            return Err("Missing argument scene item to hide".to_string())
        }
    }


    Ok(())
}