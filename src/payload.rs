use std::collections::HashMap;


use crate::obs::*;

pub struct Payload {
    opcode: OBSOpcode,
    attributes: HashMap<String, String>
}

pub enum PayloadError {
    OpCodeNotFound
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

    fn clean_up_json(mut message: String) -> String {
        message = message.replace("\"d\":", "");
        message = message.replace("{", "");
        message = message.replace("}", "");
        message
    }

    fn get_opcode(message: String) -> Result<OBSOpcode, String> {
        let mut attributes: Vec<&str> = message.split(",").collect();
        let mut op_val = "".to_string();
        for attribute in attributes {
            if attribute.contains("\"op\":") {
                op_val = attribute.replace("\"op\":", "");
            }
        }
        if op_val.is_empty() {
            return Err(Payload::error_strings(PayloadError::OpCodeNotFound));
        }

        obs_opcode_match_enum(&op_val)
    }

    fn opcode_hello_from_string(message: String) -> Result<Payload, String> {
        let mut attributes: HashMap<String, String> = HashMap::new();
        if message.contains("\"authentication\":") {

        }

        Err("".to_string())
    }

    fn opcode_identify_from_string(message: String) -> Result<Payload, String> {
        Err("".to_string())
    }

    fn opcode_identified_from_string(message: String) -> Result<Payload, String> {
        Err("".to_string())
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

    fn opcode_request_response_from_string(message: String) -> Result<Payload, String> {
        Err("".to_string())
    }

    fn opcode_request_batch_from_string(message: String) -> Result<Payload, String> {
        Err("".to_string())
    }

    fn opcode_request_batch_response_from_string(message: String) -> Result<Payload, String> {
        Err("".to_string())
    }

    pub fn error_strings(payload_error: PayloadError) -> String {
        match payload_error {
            PayloadError::OpCodeNotFound => "Opcode missing in OBS message"
        }.to_string()
    }
}