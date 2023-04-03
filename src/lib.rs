use std::net::{TcpStream, SocketAddr};
use std::io::{prelude::*, BufReader};
use std::str;

use sha2::Sha256;
use base64;

pub struct ObsConnection {
    ip: SocketAddr,
    stream: TcpStream
}

impl ObsConnection {
    pub fn new (ip: SocketAddr) -> Result<ObsConnection, String> {
        let stream = match ObsConnection::connect(&ip) {
            Ok(s) => s,
            Err(e) => return Err(e)
        };
        let mut obs_con = ObsConnection { ip, stream};
        match obs_con.init() {
            Ok(()) => (),
            Err(e) => return Err(e)
        };
        Ok(obs_con)
    }

    /// Open TCP connection
    fn connect (ip: &SocketAddr) -> Result<TcpStream, String> {
        match TcpStream::connect(ip) {
            Ok(s) => Ok(s),
            Err(e) => Err(format!("Could not connect to {}: {}", ip.to_string(), e.to_string()))
        }
    }

    ///Initialise OBS Websocket connection
    fn init(&mut self) -> Result<(), String> {
        let str = self.get_initstr();
        match self.stream.write(str.as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(format!("Could not write to {}: {}", self.ip.to_string(), e.to_string()))
        };
        let hello_payload = match self.read_hello() {
            Ok(payload) => payload,
            Err(e) => return Err(format!("Could not read Hello Message from OBS: {}", e.to_string()))
        };

        self.answer_hello(hello_payload).unwrap();

        Ok(())
    }

    ///Get initial HTTP string for upgrading to Websocket
    fn get_initstr(&self) -> String {
        let mut str = format!("GET / HTTP/1.1\r\n");
        str = format!("{str}Host: {}\r\n", self.ip.to_string());
        str = format!("{str}Upgrade: websocket\r\n");
        str = format!("{str}Connection: Upgrade\r\n");
        str = format!("{str}Sec-WebSocket-Key: AQIDBAUGBwgJCgsMDQ4PEC==\r\n"); //Todo Random
        str = format!("{str}Sec-WebSocket-Protocol: obswebsocket.json\r\n");
        str = format!("{str}Sec-WebSocket-Version: 13\r\n");
        str = format!("{str}\r\n");
        str
    }

    ///read OBS Websocket Hello message
    fn read_hello(&mut self) -> Result<String, String> {
        let mut reader = BufReader::new(&self.stream);

        //header TODO: Check values
        let mut sbuffer = String::new();
        loop {
            let mut sread = String::new();
            reader.read_line(&mut sread).unwrap();
            sbuffer += &sread;

            if sread == "\r\n" {
                break;
            }
        }
        //self.stream = reader.into_inner();

        println!("{}", sbuffer);

        let payload = match self.read_payload() {
            Ok(payload) => payload,
            Err(e) => return Err(format!("Error while reading Hello payload: {}", e))
        };

        Ok(payload)
    }

    fn answer_hello(&self, mut hello: String) -> Result<(), String> {
        hello = self.clean_up_hello_message(hello);
        let mut auth: Option<String> = None;

        //Auth needed
        if hello.contains("\"authentication\":") {
            auth = match self.get_hello_auth(hello.clone()) {
                Ok(auth) => Some(auth),
                Err(e) => return Err(format!("Error while getting authentication information from Hello message: {}", e))
            }
        }

        Ok(())
    }

    fn clean_up_hello_message(&self, mut hello: String) -> String {
        hello =hello.replace("{", "").replace("}", "");
        println!("{}\r\n", hello);
        hello = hello.replace("\"d\":", "");
        println!("{}\r\n", hello);
        hello
    }

    fn get_hello_auth (&self, mut hello: String) -> Result<String, String> {
        hello = hello.replace("\"authentication\":", "");
        println!("{}\r\n", hello);
        let valuepairs = hello.split(",");
        
        let mut challenge = String::new();
        let mut salt = String::new();

        for valuepair in valuepairs {
            if valuepair.contains("\"challenge\"") {
                let split: Vec<&str> = valuepair.split(":").collect();
                if split.len() == 2 {
                    challenge = split[1].replace("\"", "");
                }
                else {
                    return Err(format!("value of challange cannot be read. challange valuepair: {}", valuepair));
                }
            } 
            else if valuepair.contains("\"salt\"") {
                let split: Vec<&str> = valuepair.split(":").collect();
                if split.len() == 2 {
                    salt = split[1].replace("\"", "");
                }
                else {
                    return Err(format!("value of salt cannot be read. challange valuepair: {}", valuepair));
                }
            }
        }

        Ok(String::new())
    }

    fn read_payload(&mut self) -> Result<String, String> {
        //payload header TODO: check values of buffer[0]
        let mut buffer: [u8; 2] = [0; 2];
        self.stream.read(&mut buffer).unwrap();

        let length = match self.get_payload_size(buffer[1].clone()) {
            Ok(length) => length,
            Err(e) => return Err(format!("Could not get length of Hello payload: {}", e.to_string()))
        };

        println!("payload length: {}", length);

        //payload
        let mut payload = Vec::new();
        for _ in 0..length {
            let mut byte: [u8; 1] = [0; 1];
            match self.stream.read(&mut byte) {
                Ok(_) => payload.push(byte[0]),
                Err(e) => return Err(format!("Error while getting payload: {}", e.to_string()))
            }
        }
        
        match str::from_utf8(&payload) {
            Ok(string) => Ok(string.to_string()),
            Err(e) => Err(format!("Error while converting payload to utf8 string: {}", e.to_string()))
        }
    }

    fn get_payload_size(&mut self, init_size: u8) -> Result<u64, String> {
        //next 2 bytes are payload length
        if init_size == 126 {
            let mut buffer = [0; 2];
            match self.stream.read(&mut buffer) {
                Ok(_) => Ok(u16::from_be_bytes(buffer).into()),
                Err(e) => Err(e.to_string())
            }
        }
        //next 8 bytes are payload length
        else if init_size == 127 {
            let mut buffer = [0; 8];
            match self.stream.read(&mut buffer) {
                Ok(_) => Ok(u64::from_be_bytes(buffer)),
                Err(e) => Err(e.to_string())
            }
        }
        else {
            Ok(init_size.into())
        }
    }
}