use std::{collections::HashMap};

use crate::obs::*;

pub struct Payload {
    opcode: OBSOpcode,
    attributes: HashMap<String, String>
}

pub enum PayloadError {
    OpCodeNotFound,
    ParseWrongNumberOfValues,
    ParseEmptyValue,
    MissingRequiredAttribute
}

impl Payload {
    pub fn from_string(mut message: String) -> Result<Payload, String> {
        message = Payload::clean_up_json(message);
        let opcode = match Payload::get_opcode(message.clone()) {
            Ok(opcode) => opcode,
            Err(error) => return Err(error)
        };

        match opcode {
            OBSOpcode::Hello => Payload::opcode_hello_from_string(message),
            OBSOpcode::Identify => Payload::opcode_identify_from_string(message),
            OBSOpcode::Identifyed => Payload::opcode_identified_from_string(message),
            OBSOpcode::Reidentify => Payload::opcode_reidentify_from_string(message),
            OBSOpcode::Event => Payload::opcode_event_from_string(message),
            OBSOpcode::Request => Payload::opcode_request_from_string(message),
            OBSOpcode::RequestResponse => Payload::opcode_request_response_from_string(message),
            OBSOpcode::RequestBatch => Payload::opcode_request_batch_from_string(message),
            OBSOpcode::RequestBatchResponse => Payload::opcode_request_batch_response_from_string(message)
        }
    }

    pub fn opcode(&self) -> OBSOpcode {
        self.opcode
    }

    pub fn attributes(&self) -> HashMap<String, String> {
        self.attributes.clone()
    }

    fn clean_up_json(mut message: String) -> String {
        message = message.replace("\"d\":", "");
        message = message.replace("{", "");
        message = message.replace("}", "");
        message
    }

    fn get_opcode(message: String) -> Result<OBSOpcode, String> {
        let attributes: Vec<&str> = message.split(",").collect();
        let mut op_val = "".to_string();
        for attribute in attributes {
            if attribute.contains("\"op\":") {
                op_val = attribute.replace("\"op\":", "");
            }
        }
        if op_val.is_empty() {
            return Err(Payload::error_strings(PayloadError::OpCodeNotFound, Some(message)));
        }

        obs_opcode_match_enum(&op_val)
    }

    fn opcode_hello_from_string(mut message: String) -> Result<Payload, String> {
        let mut auth = false;
        if message.contains("\"authentication\":") {
            auth = true;
            message = message.replace("\"authentication\":", "");
        }

        message = message.replace("\"op\":0", "");

        let attributes = match Payload::parse_attributes(message) {
            Ok(attributes) => attributes,
            Err(error) => return Err(error)
        };

        let opcode = OBSOpcode::Hello;
        match Payload::check_opcode_hello_attributes(&attributes, auth) {
            Ok(()) => Ok(Payload{opcode, attributes}),
            Err(error) => Err(error)
        }
    }

    fn check_opcode_hello_attributes(attributes: &HashMap<String, String>, auth: bool) -> Result<(), String> {
        let mut websocketVersion = false;
        let mut rpcVersion = false;
        let mut challenge = false;
        let mut salt = false;

        //if authentication is not required
        if !auth { 
            challenge = true;
            salt = true;
        }

        for (key, value) in attributes {
            match key.as_str() {
                "obsWebSocketVersion" => websocketVersion = !value.is_empty(),
                "rpcVersion" => rpcVersion = !value.is_empty(),
                "challenge" => challenge = !value.is_empty(),
                "salt" => salt = !value.is_empty(),
                _ => ()
            }
        }

        if websocketVersion && rpcVersion && challenge && salt {
            return Ok(())
        }
        else {
            let mut atts = "Opcode 0 missing ".to_string();

            if !websocketVersion {
                atts = format!("{atts} \"obsWebSocketVersion\" ");
            }
            if !rpcVersion {
                atts = format!("{atts} \"rpcVersion\" ");
            }
            if !challenge {
                atts = format!("{atts} \"challenge\" ");
            }
            if !salt {
                atts = format!("{atts} \"salt\" ");
            }

            atts = format!("{atts}. Provided values: ");

            for (key, value) in attributes {
                atts = format!("{atts} {key}, {value};");
            }
            return Err(Payload::error_strings(PayloadError::MissingRequiredAttribute, Some(atts)));
        }
    }

    fn opcode_identify_from_string(message: String) -> Result<Payload, String> {
        Err("".to_string())
    }

    pub fn opcode_identify_message(rpcversion: usize, authentication: Option<String>, event_subscriptions: OBSEventSubscription) -> String{
        let mut message = "{".to_string();
        message = message + "\"op\":1,";
        message = message + "\"d\":{";
        message = format!("{message}\"rpcVersion\":{rpcversion},");
        match authentication {
            Some(auth) => message = format!("{}\"authentication\":\"{}\",", message, auth),
            None => ()
        }
        message = format!("{message}\"eventSubscriptions\":{}", obs_event_subscription_match_value(event_subscriptions));
        message = message + "}}";

        println!("{message}");//Debug
        message.to_string()
    }

    fn opcode_identified_from_string(mut message: String) -> Result<Payload, String> {
        message = message.replace("\"op\":2", "");
        println!("{message}");

        let attributes = match Payload::parse_attributes(message) {
            Ok(attributes) => attributes,
            Err(error) => return Err(error)
        };

        let opcode = OBSOpcode::Identifyed;
        match Payload::check_opcode_identified_attributes(&attributes) {
            Ok(()) => Ok(Payload {opcode, attributes}),
            Err(error) => Err(error)
        }
    }

    fn check_opcode_identified_attributes(attributes: &HashMap<String, String>) -> Result<(), String> {
        let mut negotiated_rpcversion = false;

        for (key, value) in attributes {
            match key.as_str() {
                "negotiatedRpcVersion" => negotiated_rpcversion = !value.is_empty(),
                _ => ()
            }
        }

        if negotiated_rpcversion {
            Ok(())
        }
        else {
            Err(Payload::error_strings(PayloadError::MissingRequiredAttribute, Some("Opcode 2 missing \"negotiatedRpcVersion\"".to_string())))
        }
    }

    fn opcode_reidentify_from_string(message: String) -> Result<Payload, String> {
        Err("".to_string())
    }

    fn opcode_event_from_string(message: String) -> Result<Payload, String> {
        Err("".to_string())
    }

    fn opcode_request_from_string(message: String) -> Result<Payload, String> {
        Err("".to_string())
    }

    fn opcode_request_response_from_string(mut message: String) -> Result<Payload, String> {
        message = message.replace("\"op\":7", "");
        message = message.replace("\"requestStatus\":", "");
        message = message.replace("\"responseData\":", "");
        println!("{message}");

        let opcode = OBSOpcode::Request;
        match Payload::parse_attributes(message) {
            Ok(attributes) => Ok(Payload{attributes, opcode}),
            Err(error) => Err(error)
        }
    }

    fn opcode_request_batch_from_string(message: String) -> Result<Payload, String> {
        Err("".to_string())
    }

    fn opcode_request_batch_response_from_string(message: String) -> Result<Payload, String> {
        Err("".to_string())
    }

    fn parse_attributes(mut message: String) -> Result<HashMap<String, String>, String>{
        message = message.replace("\"", "");
        let mut attributes: HashMap<String, String> = HashMap::new();
        let split: Vec<&str> = message.split(",").collect();

        for attribute in split {
            if !attribute.is_empty() {
                let values: Vec<&str> = attribute.split(":").collect();
                if values.len() == 2 {
                    if !values[0].is_empty() && !values[1].is_empty() {
                        attributes.insert(values[0].to_string(), values[1].to_string());
                    }
                    else {
                        
                        return Err(Payload::error_strings(PayloadError::ParseEmptyValue, Some(attribute.to_string())))
                    }
                }
                else {
                    let mut str = attribute.to_string();
                    str = format!("{str}, {}", values.len());
                        for val in values {
                            str = format!("{str}, {}", val);
                        }
                    return Err(Payload::error_strings(PayloadError::ParseWrongNumberOfValues, Some(str)))
                }
            }   
        }

        Ok(attributes)
    }

    pub fn error_strings(payload_error: PayloadError, information: Option<String>) -> String {
        let message = match payload_error {
            PayloadError::OpCodeNotFound => "Opcode missing in OBS message",
            PayloadError::ParseWrongNumberOfValues => "Parsing a attribute resulted in more or less than 2 values",
            PayloadError::ParseEmptyValue => "A parsed attribute is empty",
            PayloadError::MissingRequiredAttribute => "A required attribute is missing",
        }.to_string();

        match information {
            Some(info) => format!("{}: {}", message, info),
            None => message
        }
    }
}