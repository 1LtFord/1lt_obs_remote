pub struct Header {
    fin: bool,
    opcode: Opcode,
    mask: bool,
    payload_length: u64
}

pub enum Opcode {
    ContinuationFrame,
    TextFrame,
    BinaryFrame,
    ConnectionClose,
    Ping,
    Pong
}

pub enum HeaderError {
    InsufficientHeaderData,
    InsufficientPayloadLengthData,
    CustomOpcodeNotSupported
}

impl Header {
    pub fn new (fin: bool, opcode: Opcode, mask: bool, payload_length: u64) -> Header {
        Header{fin, opcode, mask, payload_length}
    }

    pub fn from_bytes(bytes: &Vec<u8>) -> Result<Header, HeaderError> {
        match Header::check_bytes(bytes) {
            Ok(()) => (),
            Err(error) => return Err(error)
        };

        let fin = bytes[0] >= 128;
        let opcode = match Header::opcode_match_enum(bytes[0]) {
            Ok(value) => value,
            Err (error) => return Err(error)
        };
        let mask = bytes[1] >= 128;
        let payload_length = Header::get_payload_length(&bytes);

        Ok(Header{fin, opcode, mask, payload_length})
    }

    ///check if vector of bytes fulfills all criteria to be correctly interpreted as a WebSocket header
    fn check_bytes(bytes: &Vec<u8>) -> Result<(), HeaderError> {
        //check if vector has minimum header size
        if bytes.len() > 2 {
            let minimum_header: [u8; 2] = [bytes[0], bytes[1]];

            //check if vector has required header size
            if bytes.len() >= Header::required_header_size(minimum_header) {
                return Ok(());
            }
            else {
                return Err(HeaderError::InsufficientPayloadLengthData)
            }
        }
        else {
            return Err(HeaderError::InsufficientHeaderData);
        }
    }

    fn get_payload_length (bytes: &Vec<u8>) -> u64 {
        let minimum_header: [u8; 2] = [bytes[0], bytes[1]];
        let header_size = Header::required_header_size(minimum_header);
        let mut payload_length: u64 = 0;

        if header_size == 2 {
            if bytes[1] >= 128 {
                payload_length = (bytes[1] - 128) as u64;
            }
            else {
                payload_length = bytes[1] as u64;
            }
        }
        else if header_size == 4 {
            payload_length = u16::from_be_bytes([bytes[2], bytes[3]]) as u64;
        }
        else if header_size == 10 {
            payload_length = u64::from_be_bytes([bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7], bytes[8], bytes[9]]);
        }

        payload_length
    }

    ///Returns required header size in bytes, calculated from the first 2 bytes of the header
    pub fn required_header_size (bytes: [u8; 2]) -> usize {
        let mut byte = bytes[1].clone();

        //remove Mask bit
        if byte >= 128 {
            byte = byte - 128;
        }

        //match required header size in byte
        match byte {
            126 => 4,
            127 => 10,
            other => 2
        }
    }

    /// Get Opcode value from Enum
    pub fn opcode_match_value(opcode: Opcode) -> u8 {
        match opcode {
            Opcode::ContinuationFrame => 0,
            Opcode::TextFrame => 1,
            Opcode::BinaryFrame => 2,
            Opcode::ConnectionClose => 8,
            Opcode::Ping => 9,
            Opcode::Pong => 10
        }
    }
    
    /// Get Opcode enum from Value
    /// Accepts full first 8 bits from Websocket header
    pub fn opcode_match_enum(mut opcode: u8) -> Result<Opcode, HeaderError> {
        opcode = opcode % 16;
        match opcode {
            0 => Ok(Opcode::ContinuationFrame),
            1 => Ok(Opcode::TextFrame),
            2 => Ok(Opcode::BinaryFrame),
            8 => Ok(Opcode::ConnectionClose),
            9 => Ok(Opcode::Ping),
            10 => Ok(Opcode::Pong),
            _ => Err(HeaderError::CustomOpcodeNotSupported)
        }
    }
}


