use std::{
    io::{self, Read},
    net::TcpStream,
};

use crate::{constants::MsgId, download::download::PieceProgress, log::debug};

#[derive(Debug, PartialEq)]
pub struct Message {
    pub id: u8,
    pub payload: Vec<u8>,
}

impl Message {
    pub fn have(self) -> Result<u32, String> {
        if self.id != MsgId::Have.to_u8() {
            return Err(format!(
                "expected HAVE: {}, got id: {}",
                MsgId::Have.to_u8(),
                self.id
            ));
        }
        debug(format!("+++++++++++++++++++++ {}", self.payload.len()));
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
        if self.id != MsgId::Piece.to_u8() {
            return Err(format!(
                "expected HAVE: {}, got id: {}",
                MsgId::Piece.to_u8(),
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
        if (begin as usize) >= progress.buf.len() {
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
    match con.read_exact(&mut len_buf) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == io::ErrorKind::TimedOut {
                return Err(String::from("read operation timed out!"));
            } else {
                // sometimes causes network errors
                // Resource temporarily unavailable (os error 11)
                // failed to fill whole buffer
                //return Err(format!("{}, {:?}", e.to_string(), con.peer_addr()));
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
    match con.read_exact(&mut msg_buf) {
        // sometimes causes errors
        // cant read full buffer
        Ok(_) => {}
        Err(e) => {
            if e.kind() == io::ErrorKind::TimedOut {
                return Err(String::from("read operation timed out!"));
            } else {
                return Err(e.to_string());
            }
        }
    };

    // here we can read msg id and ignore it if it is none of MsgIds
    match msg_buf[0] {
        7 => {
            //println!("+++++++++++PIECE msg buf: {:?}", &msg_buf[0..20]);
        }
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 8 => {}
        _ => {
            return Err(format!("unacceptable message id: {}", msg_buf[0]));
        }
    }

    //println!(
    //    "------------------------size:{:?}/{}, id: {}, len:{}",
    //    len_buf,
    //    len,
    //    msg_buf[0],
    //    msg_buf.len()
    //);
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
            id: MsgId::Have.to_u8(),
            payload: vec![0, 0, 0, 1],
        };

        let result = message.have();
        assert_eq!(result, Ok(1));
    }
    #[test]
    fn test_have_wrong_id() {
        let message = Message {
            id: MsgId::Interested.to_u8(),
            payload: vec![0, 0, 0, 1],
        };

        let result = message.have();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "expected HAVE: 4, got id: 2");
    }
    #[test]
    fn test_have_wrong_payload_length() {
        let message = Message {
            id: MsgId::Have.to_u8(),
            payload: vec![0, 0],
        };

        let result = message.have();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "expected length to be 4 got: 2");
    }
    #[test]
    fn test_have_payload_conversion_failure() {
        let message = Message {
            id: MsgId::Have.to_u8(),
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
            id: MsgId::Have.to_u8(),
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
