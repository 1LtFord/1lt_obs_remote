use std::{net::{SocketAddr, TcpStream}, io::{Write, Read, BufReader, BufRead}, time::Duration};

use rand::{RngCore, Rng};
use base64::{Engine as _, engine::general_purpose};
use sha1::{Sha1, Digest};
use sha2::Sha256;

use crate::{message::Message, header::Header, payload::Payload, obs::{OBSOpcode, OBSEventSubscription}};

pub struct Websocket {
    ip: SocketAddr,
    stream: TcpStream,
    state: State,
}

pub enum State {
    initialising,
    open,
    closed
}

pub enum WebsocketError {
    ConnectError,
    SetReadTimeoutError,
    SetWriteTimeoutError,
    WriteError,
    ReadError,
    UpgradeError,
    UpgradeTypeError,
    UpgradeConncetionError,
    UpgradeAcceptError,
    UpgradeProtocolError,
    MissingOpCode0,
    RpcVersionNotSupported,
    AuthenticationRequired,
    NotIdentified
}

impl Websocket {
    pub fn new(ip: SocketAddr, password: Option<String>, rpcversion: usize) -> Result<Websocket, String> {
        //initialise Websocket
        let mut websocket = match Websocket::initialise_websocket(ip) {
            Ok(websocket) => websocket,
            Err(error) => return Err(error)
        };

        //OBS Protocol initialisation
        //read opcode 0 | Hello
        let hello_message = match websocket.read_message() {
            Ok(message) => message,
            Err(error) => return Err(error)
        };
        println!("{}", hello_message.payload_value()); //debug
        let payload = match Payload::from_string(hello_message.payload_value()) {
            Ok(payload) => payload,
            Err(error) => return Err(error)
        };
        if payload.opcode() != OBSOpcode::Hello {
            return Err(Websocket::error_strings(WebsocketError::MissingOpCode0, None));
        }

        //send opcode 1 | Identify
        let opcode1_payload = match Websocket::opcode1_payload(rpcversion, password, OBSEventSubscription::None, payload) {
            Ok(message) => message,
            Err(error) => return Err(error)
        };
        match websocket.send_message(Message::new(true, crate::header::Opcode::TextFrame, true, opcode1_payload)) {
            Ok(()) => (),
            Err(error) => return Err(error)
        }

        //read opcode 2| Identified
        let identified_message = match websocket.read_message() {
            Ok(message) => message,
            Err(error) => return Err(error)
        };
        println!("{}", identified_message.payload_value());

        let identified_payload = match Payload::from_string(identified_message.payload_value()) {
            Ok(payload) => payload,
            Err(error) => return Err(error)
        };

        if identified_payload.opcode() == OBSOpcode::Identifyed {
            Ok(websocket)
        }
        else {
            Err(Websocket::error_strings(WebsocketError::NotIdentified, Some(identified_message.payload_value())))
        }
    }

    //initialise
    fn initialise_websocket(ip: SocketAddr) -> Result<Websocket, String> {
        let sec_websocket_key = Websocket::new_sec_websocket_key();
        let mut stream = match TcpStream::connect_timeout(&ip, Duration::from_secs(5)) {
            Ok(stream) => stream,
            Err(error) => return Err(Websocket::error_strings(WebsocketError::ConnectError, Some(format!("Could not connect to {}: {}", ip.to_string(), error.to_string()))))
        };

        //Set timeouts
        match stream.set_read_timeout(Some(Duration::from_secs(5))) {
            Ok(()) => (),
            Err(error) => return Err(Websocket::error_strings(WebsocketError::SetReadTimeoutError, Some(error.to_string())))
        }
        match stream.set_write_timeout(Some(Duration::from_secs(5))) {
            Ok(()) => (),
            Err(error) => return Err(Websocket::error_strings(WebsocketError::SetWriteTimeoutError, Some(error.to_string()))) 
        }

        //Upgrade connection
        match stream.write(Websocket::request_upgrade_message(ip, sec_websocket_key.clone()).as_bytes()) {
            Ok(_) => (),
            Err(error) => return Err(Websocket::error_strings(WebsocketError::WriteError, Some(format!("Could not write to {}: {}", ip.to_string(), error.to_string()))))
        };
        match Websocket::read_upgrade_response(&mut stream, sec_websocket_key.clone()) {
            Ok(()) => (),
            Err(error) => return Err(error)
        };

        let state = State::initialising;
        Ok(Websocket{ip, stream, state})
    }

    fn request_upgrade_message(ip: SocketAddr, sec_websocket_key: String) -> String {
        let mut str = format!("GET / HTTP/1.1\r\n");
        str = format!("{str}Host: {}\r\n", ip.to_string());
        str = format!("{str}Upgrade: websocket\r\n");
        str = format!("{str}Connection: Upgrade\r\n");
        str = format!("{str}Sec-WebSocket-Key: {sec_websocket_key}\r\n");
        str = format!("{str}Sec-WebSocket-Protocol: obswebsocket.json\r\n");
        str = format!("{str}Sec-WebSocket-Version: 13\r\n");
        str = format!("{str}\r\n");
        println!("{}", str); //Debug
        str
    }

    fn read_upgrade_response(stream: &mut TcpStream, sec_websocket_key: String) -> Result<(), String> {
        let mut reader = BufReader::new(stream);

        let mut sbuffer = String::new();
        loop {
            let mut sread = String::new();
            match reader.read_line(&mut sread) {
                Ok(_) => (),
                Err(error) => return Err(Websocket::error_strings(WebsocketError::ReadError,Some(format!("Could not read upgrade request response: {}", error.to_string()))))
            }
            sbuffer += &sread;

            if sread == "\r\n" {
                break;
            }
        }

        println!("{}", sbuffer); //Debug
        Websocket::confirm_upgrade_response(sbuffer, sec_websocket_key)
    }

    fn confirm_upgrade_response(message: String, sec_websocket_key: String) -> Result<(), String> {
        let lines: Vec<&str> = message.split("\r\n").collect();

        //Check for HTTP Status
        if lines[0] != "HTTP/1.1 101 Switching Protocols" {
            if lines[0].starts_with("HTTP/1.1") {
                return Err(Websocket::error_strings(WebsocketError::UpgradeError, Some(lines[0].to_string())));
            }
            else {
                return Err(Websocket::error_strings(WebsocketError::UpgradeError, Some(message)));
            }
            
        }

        match Websocket::check_upgrade_response_values(lines, sec_websocket_key) {
            Ok(()) => Ok(()),
            Err(error) => Err(error)
        }
    }

    fn check_upgrade_response_values(lines: Vec<&str>, sec_websocket_key: String) -> Result<(), String> {
        let mut websocket = false;
        let mut upgrade = false;
        let mut accept = false;
        let mut protocol = false;

        for line in &lines[1..] {
            match line {
                &"Upgrade: websocket" => {websocket = true; continue;},
                &"Connection: Upgrade" => {upgrade = true; continue;},
                _ => ()
            }
            if line.starts_with("Sec-WebSocket-Accept:") {
                accept = Websocket::check_sec_websocket_accept(line.replace("Sec-WebSocket-Accept: ", ""), sec_websocket_key.clone());
                continue;
            }
            if line.starts_with("Sec-WebSocket-Protocol:") {
                if line.replace("Sec-WebSocket-Protocol: ", "") == "obswebsocket.json" {
                    protocol = true;
                }
                continue;
            }
        }

        if !websocket {
            return Err(Websocket::error_strings(WebsocketError::UpgradeTypeError, None));
        }
        if !upgrade {
            return Err(Websocket::error_strings(WebsocketError::UpgradeConncetionError, None));
        }
        if !accept {
            return Err(Websocket::error_strings(WebsocketError::UpgradeAcceptError, None));
        }
        if !protocol {
            return Err(Websocket::error_strings(WebsocketError::UpgradeProtocolError, None));
        }

        Ok(())
    }
    
    fn new_sec_websocket_key() -> String {
        let mut random =  [0u8; 16];
        rand::thread_rng().fill_bytes(&mut random);
        general_purpose::STANDARD.encode(random)
    }

    fn check_sec_websocket_accept(sec_websocket_accept: String, sec_websocket_key: String) -> bool {
        //recreate supposed sec_websocket_accept value
        let fixed = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11".to_string();
        let mut check_value = format!("{}{}", sec_websocket_key, fixed);
        let mut hasher = Sha1::new();
        hasher.update(check_value);
        check_value = general_purpose::STANDARD.encode(hasher.finalize());

        sec_websocket_accept == check_value
    }

    fn opcode1_payload(rpcversion: usize, password: Option<String>, event_subscriptions: OBSEventSubscription, opcode0_message: Payload) -> Result<String, String> {
        let mut auth_required = false;
        let mut challenge = String::new();
        let mut salt = String::new();
        let mut rpcversion_supported = false;
        let mut requested_rpcversion = String::new();
        

        for (key, value) in opcode0_message.attributes() {
            match key.as_str() {
                "rpcVersion" => {rpcversion_supported = Websocket::rpcVersion_supported(value.clone(), rpcversion); requested_rpcversion = value},
                "challenge" => {challenge = value; auth_required = true},
                "salt" => {salt = value; auth_required = true},
                _ => ()
            }
        }

        if !rpcversion_supported {
            return Err(Websocket::error_strings(WebsocketError::RpcVersionNotSupported, Some(format!("requested {requested_rpcversion}, supported {rpcversion}"))));
        }

        if auth_required {
            let authentication = match password {
                Some(password) => Websocket::create_authentication_string(password, challenge, salt),
                None => return Err(Websocket::error_strings(WebsocketError::AuthenticationRequired, None))
            };
            Ok(Payload::opcode_identify_message(1, Some(authentication), event_subscriptions))
        }
        else {
            Ok(Payload::opcode_identify_message(rpcversion, None, event_subscriptions))
        }
        
        
    }

    fn create_authentication_string(password: String, challenge: String, salt: String) -> String {
        let secret = Websocket::create_base64_secret(format!("{password}{salt}"));
        Websocket::create_base64_secret(format!("{secret}{challenge}"))
    }

    fn create_base64_secret(text: String) -> String {
        let mut hasher = Sha256::new();
        hasher.update(text);
        let hash = hasher.finalize();
        general_purpose::STANDARD.encode(hash)
    }

    fn rpcVersion_supported(rpcVersion: String, supported: usize) -> bool {
        let rpcv: usize = match rpcVersion.parse() {
            Ok(rpcv) => rpcv,
            Err(error) => return false
        };

        rpcv == supported
    }
    //-----------

    pub fn send_message(&mut self, message: Message) -> Result<(), String> {
        match self.stream.write(&message.to_bytes()) {
            Ok(_) => Ok(()),
            Err(error) => Err(Websocket::error_strings(WebsocketError::WriteError, Some(format!("Could not write to Websocket: {}", error.to_string()))))
        }
    }

    pub fn read_message(&mut self) -> Result<Message, String> {
        //read header
        let mut bytes = match self.read_header() {
            Ok(bytes) => bytes,
            Err(error) => return Err(error)
        };

        //read mask if set
        let header = match Header::from_bytes(bytes.clone()) {
            Ok(header) => header,
            Err(error) => return Err(Header::error_strings(error))
        };
        if header.has_mask_byte_set() {
            match self.read_mask() {
                Ok(mask) => bytes.extend(mask.into_iter()),
                Err(error) => return Err(error)
            };
        }
        
        //read payload
        match self.read_payload(header.get_value_payload_length()) {
            Ok(payload) => bytes.extend(payload.iter()),
            Err(error) => return Err(error)
        };
        
        //parse message
        Ok(match Message::from_bytes(bytes) {
            Ok(message) => message,
            Err(error) => return Err(error)
        })
    }

    fn read_header(&mut self) -> Result<Vec<u8>, String> {
        let mut header: Vec<u8> = Vec::new();
        let mut buffer = [0u8; 2];
        match self.read(&mut buffer) {
            Ok(buffer) => header.extend(buffer.iter()),
            Err(error) => return Err(error)
        }

        //check for extended header size
        let header_size = Header::required_header_size(buffer.clone());
        if header_size != 2 {
            if header_size == 4 {
                let extra = [0u8; 2];
                match self.read(&mut buffer) {
                    Ok(buffer) => header.extend(buffer.iter()),
                    Err(error) => return Err(error)
                }
            }
            else if header_size == 10 {
                let extra = [0u8; 8];
                match self.read(&mut buffer) {
                    Ok(buffer) => header.extend(buffer.iter()),
                    Err(error) => return Err(error)
                }
            }
        }

        Ok(header)
    }

    fn read_mask(&mut self) -> Result<Vec<u8>, String> {
        let mut mask: Vec<u8> = Vec::new();
        let mut buffer = [0u8; 4];
        match self.read(&mut buffer) {
            Ok(buffer) => mask.extend(buffer.iter()),
            Err(error) => return Err(error)
        }

        Ok(mask)
    }

    fn read_payload(&mut self, payload_length: u64) -> Result<Vec<u8>, String> {
        let mut payload: Vec<u8> = Vec::new();
        for _ in 0..payload_length {
            let mut byte = [0u8; 1];
            match self.read(&mut byte) {
                Ok(byte) => payload.push(byte[0]),
                Err(error) => return Err(error)
            };
        }

        Ok(payload)
    }

    fn read<'a>(&'a mut self, buffer: &'a mut [u8]) -> Result<&[u8], String> {
        match self.stream.read(buffer) {
            Ok(_) => Ok(buffer),
            Err(error) => Err (Websocket::error_strings(WebsocketError::ReadError, Some(error.to_string())))
        }
    }

    //debug
    /*fn read_close(&mut self) -> Result<String, String> {
        let mut message: Vec<char> = Vec::new();
        loop {
            let mut byte = [0u8; 1];
            match self.read(&mut byte) {
                Ok(byte) => message.push(byte[0] as char),
                Err(_) => break
            };
            print!("{}", byte[0] as char);
            if(message.len() >= 4) {
                if format!("{}{}", message[message.len()-1], message[message.len()-2]) == "\r\n" && format!("{}{}",message[message.len()-3],message[message.len()-4]) == "\r\n" {
                    break;
                }
            }
        }

        Ok(String::from_iter(message))
    }*/

    fn error_strings(error: WebsocketError, informations: Option<String>) -> String {
        let string = match error {
            WebsocketError::ConnectError => "Error while connecting to WebSocket server",
            WebsocketError::SetReadTimeoutError => "Could not set connection read timeout",
            WebsocketError::SetWriteTimeoutError => "Could not set connection write timeout",
            WebsocketError::WriteError => "Error while writing to WebSocket",
            WebsocketError::ReadError => "Error while reading from Websocket",
            WebsocketError::UpgradeError => "Could not upgrade connection",
            WebsocketError::UpgradeTypeError => "Could not upgrade connection. Upgrade type wrong/not found",
            WebsocketError::UpgradeConncetionError => "Could not upgrade connection. Upgrade connection indicator wrong/not found",
            WebsocketError::UpgradeAcceptError => "Could not upgrade connection. Upgrade accept value wrong/not found",
            WebsocketError::UpgradeProtocolError => "Could not upgrade connection. Upgrade protocol wrong/ not found",
            WebsocketError::MissingOpCode0 => "Wrong Message recieved. Expected Opcode 0",
            WebsocketError::RpcVersionNotSupported => "The submitted RpcVersion is not supported",
            WebsocketError::AuthenticationRequired => "This OBS WebSocket requires authentication",
            WebsocketError::NotIdentified => "Could not authenticate"
        }.to_string();
        match informations {
            Some(info) => format!("{}: {}", string, info),
            None => string
        }
    }
}

