use crate::header::Header;

pub struct Message {
    header: Header,
    mask: [u8; 4],
    payload: String
}