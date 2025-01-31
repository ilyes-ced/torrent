use std::{
    io::{self, Read},
    net::TcpStream,
};

use crate::constants::MsgId;

use super::download::PieceProgress;

#[derive(Debug, PartialEq)]
pub struct Message {
    pub id: u8,
    pub payload: Vec<u8>,
}

impl Message {
    pub fn have(self) -> Result<u32, String> {
        if self.id != MsgId::HAVE.to_u8() {
            return Err(format!(
                "expected HAVE: {}, got id: {}",
                MsgId::HAVE.to_u8(),
                self.id
            ));
        }
        println!("+++++++++++++++++++++ {}", self.payload.len());
        if self.payload.len() != 4 {
            return Err(format!(
                "expected length to be 4 got: {}",
                self.payload.len()
            ));
        }

        let bytes: [u8; 4] = match self.payload.try_into() {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(format!(
                    "Payload conversion failed; expected 4 bytes but got {}",
                    err.len()
                ))
            }
        };

        Ok(u32::from_be_bytes(bytes))
    }

    pub fn parse_piece(self, progress: &PieceProgress) -> Result<(Vec<u8>, u32), String> {
        if self.id != MsgId::PIECE.to_u8() {
            return Err(format!(
                "expected HAVE: {}, got id: {}",
                MsgId::PIECE.to_u8(),
                self.id
            ));
        }
        if self.payload.len() < 8 {
            return Err(format!(
                "expected length to be more than 8 got: {}",
                self.payload.len()
            ));
        }
        let index = u32::from_be_bytes(self.payload[0..4].try_into().unwrap());
        if progress.index != index {
            return Err(format!(
                "expected index: {}, got: {}",
                progress.index, index
            ));
        }
        let begin = u32::from_be_bytes(self.payload[4..8].try_into().unwrap());
        if begin >= progress.buf.len().try_into().unwrap() {
            return Err(format!(
                "begin offset beyond whats available {} >= {}",
                begin,
                progress.buf.len()
            ));
        }
        let block = self.payload[8..].to_vec();
        if (begin as usize + block.len()) > progress.buf.len() {
            return Err(format!(
                "data too long {} for offset {} with length {}",
                block.len(),
                begin,
                progress.buf.len()
            ));
        }
        std::fs::write("parsed_piece.txt", format!("{:?}", block));

        let mut file = std::fs::OpenOptions::new()
            .write(true) // Enable writing
            .append(true) // Enable appending
            .open("parsed_piece.txt")
            .unwrap(); // Specify your file name

        // Write data to the file
        io::Write::write_all(&mut file, block.as_slice()).unwrap(); // Convert string to bytes

        Ok((block, begin))
    }
}

pub fn to_buf(msg: Option<Message>) -> Vec<u8> {
    match msg {
        Some(msg) => {
            let length = (msg.payload.len() + 1) as u32;
            let mut buf = vec![0; 4 + length as usize];
            buf[0..4].copy_from_slice(&length.to_be_bytes());
            buf[4] = msg.id;
            buf[5..].copy_from_slice(&msg.payload);
            buf
        }
        None => vec![0; 4],
    }
}

// reads message from the connection with a peer
pub fn from_buf(mut con: &TcpStream) -> Result<Message, String> {
    // reads first 4 bytes = lenght of msg
    let mut len_buf = [0; 4];
    match con.read(&mut len_buf) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == io::ErrorKind::TimedOut {
                return Err(String::from("read operation timed out!"));
            } else {
                return Err(e.to_string());
            }
        }
    };

    let len = u32::from_be_bytes(len_buf);
    if len == 0 {
        return Err(String::from("keep alive signal"));
    }

    // reads the rest of the message: id + payload
    let mut msg_buf: Vec<u8> = vec![0; len as usize];
    if len != msg_buf.len() as u32 {
        return Err(format!(
            "payload lenght and message Length does no match, len: {}, payload+4: {}",
            len,
            msg_buf.len() + 4,
        ));
    }
    match con.read(&mut msg_buf) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == io::ErrorKind::TimedOut {
                return Err(String::from("read operation timed out!"));
            } else {
                return Err(e.to_string());
            }
        }
    };

    Ok(Message {
        id: msg_buf[0],
        payload: msg_buf[1..].to_vec(),
    })
}

//
//
//
// tests
// idk how to test for stuff with connections
//
//

#[cfg(test)]
mod tests {
    use super::*;
    // have tests
    #[test]
    fn test_have_success() {
        let message = Message {
            id: MsgId::HAVE.to_u8(),
            payload: vec![0, 0, 0, 1],
        };

        let result = message.have();
        assert_eq!(result, Ok(1));
    }
    #[test]
    fn test_have_wrong_id() {
        let message = Message {
            id: MsgId::INTRESTED.to_u8(),
            payload: vec![0, 0, 0, 1],
        };

        let result = message.have();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "expected HAVE: 4, got id: 2");
    }
    #[test]
    fn test_have_wrong_payload_length() {
        let message = Message {
            id: MsgId::HAVE.to_u8(),
            payload: vec![0, 0],
        };

        let result = message.have();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "expected length to be 4 got: 2");
    }
    #[test]
    fn test_have_payload_conversion_failure() {
        let message = Message {
            id: MsgId::HAVE.to_u8(),
            payload: vec![0; 5],
        };

        let result = message.have();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "expected length to be 4 got: 5");
    }

    // parse_piece tests
    #[test]
    fn request() {
        let message = Message {
            id: MsgId::HAVE.to_u8(),
            payload: vec![0, 0, 0, 1],
        };

        let result = message.have();
        assert_eq!(result, Ok(1));
    }
}

//
//mod tests {
//    use super::*;
//
//    #[test]
//    fn to_buffer() {
//        let result = to_buf(Some(Message {
//            id: 5,
//            payload: [90, 90, 90].to_vec(),
//        }));
//        assert_eq!(result, Vec::from([0, 0, 0, 4, 5, 90, 90, 90]));
//    }
//
//    #[test]
//    fn to_buffer_none() {
//        let result = to_buf(None);
//        assert_eq!(result, Vec::from([0, 0, 0, 0]));
//    }
//
//    #[test]
//    fn from_buffer() {
//        let result = from_buf(Vec::from([
//            0, 0, 0, 10, 5, 90, 90, 90, 25, 69, 7, 45, 55, 2,
//        ]))
//        .unwrap();
//        assert_eq!(
//            result,
//            Some(Message {
//                id: 5,
//                payload: Vec::from([90, 90, 90, 25, 69, 7, 45, 55, 2,])
//            })
//        );
//    }
//
//    #[test]
//    fn from_buffer_none() {
//        let result = from_buf(Vec::from([0, 0, 0, 0])).unwrap();
//        assert_eq!(
//            result,
//            Some(Message {
//                id: 5,
//                payload: Vec::from([90, 90, 90, 25, 69, 7, 45, 55, 2,])
//            })
//        );
//    }
//}
//
