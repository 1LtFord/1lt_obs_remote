use rand::RngCore;

use crate::header::{Header, Opcode};

pub struct Message {
    header: Header,
    mask: Option<[u8; 4]>,
    payload: String
}

pub enum MessageError {
    InsufficientDataMask,
    InsufficientDataPayload
}

impl Message {
    pub fn new(fin: bool, opcode: Opcode, masked: bool, payload: String) -> Message {
        let header = Header::new(fin, opcode, masked, payload.as_bytes().len() as u64);
        let mask = if masked {Some(Message::create_random_mask())} else {None};
        Message{header, mask, payload}
    }

    pub fn from_bytes (bytes: Vec<u8>) -> Result<Message, String> {
        //get header
        let header = match Header::from_bytes(bytes.clone()) {
            Ok(h) => h,
            Err(e) => return Err(Header::error_strings(e)) 
        };
        //get header length for bytes to skip when getting payload
        let mut skip_bytes = Header::required_header_size([bytes[0], bytes[1]]);
        
        let mut mask: Option<[u8; 4]> = None;
        //get mask if set
        if header.has_mask_byte_set() {
            mask = match Message::get_mask_from_bytes(bytes.clone(), skip_bytes.clone()) {
                Ok(mask) => Some(mask),
                Err(e) => return Err(Message::error_strings(e))
            };
            //add mask length to bytes to skip if set
            skip_bytes += 4;
        }
        
        let payload = match Message::get_payload_string_from_bytes(bytes.clone(), skip_bytes, mask.clone()) {
            Ok(payload) => payload,
            Err(e) => Message::error_strings(e)
        };

        Ok(Message{header, mask, payload})
    }

    pub fn to_bytes (&self) -> Vec<u8> {
        let payload = self.payload.as_bytes().to_vec();
        let masked_payload = match self.mask {
            Some(mask) => Message::mask_payload(payload, mask),
            None => payload
        };

        let mut message: Vec<u8> = self.header.to_bytes();
        match self.mask {
            Some(mask) => message.extend_from_slice(&mask.to_vec()),
            None => ()
        }
        message.extend_from_slice(&masked_payload);

        message
    }

    pub fn payload_value(&self) -> String {
        self.payload.clone()
    }

    fn mask_payload(payload: Vec<u8>, mask: [u8; 4]) -> Vec<u8> {
        let mut masked_payload: Vec<u8> = Vec::new();
        let index: usize = 0;
        for byte in payload {
            masked_payload.push(byte ^ mask[(index % 4) as usize]);
        }
        masked_payload
    }

    fn get_mask_from_bytes(bytes: Vec<u8>, header_length: usize) -> Result<[u8; 4], MessageError>{
        if bytes.len() >= (header_length+4) {
             let mask = bytes[header_length..header_length+4].to_vec();
             Ok([mask[0], mask[1], mask[2], mask[3]])
        }
        else {
            Err(MessageError::InsufficientDataMask)
        }
    }

    fn get_payload_string_from_bytes(bytes: Vec<u8>, skip: usize, mask: Option<[u8; 4]>) -> Result<String, MessageError> {
        if bytes.len() >= skip {
            let mut chars: Vec<char> = Vec::new();

            let mut index: u64 = 0;
            for byte in bytes[skip..].to_vec() {
                let unmasked_byte = match mask {
                    Some(mask) => byte ^ mask[(index % 4) as usize],
                    None => byte
                };
                
                chars.push(unmasked_byte as char);
                index += 1;
            }

            Ok(String::from_iter(chars))
        }
        else {
            Err(MessageError::InsufficientDataPayload)
        }
    }

    fn create_random_mask() -> [u8; 4] {
        let mut mask = [0u8; 4];
        rand::thread_rng().fill_bytes(&mut mask);
        mask
    }

    pub fn error_strings(error: MessageError) -> String {
        match error {
            MessageError::InsufficientDataMask => "Insufficient data to parse the mask of this message".to_string(),
            MessageError::InsufficientDataPayload => "Insufficient data to parse the payload of this message".to_string()
        }
    }
}