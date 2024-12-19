use serde_json::{json, Value};

enum DataType {
    Dictionary,
    List,
    String,
    Int,
}

pub struct Decoder {
    input: Vec<u8>,
    result: Value,
    cursor: usize,
    finished: bool,
}

impl Decoder {
    fn new(input: &[u8]) -> Decoder {
        Decoder {
            input: input.to_vec(),
            result: json!({}),
            cursor: 0,
            finished: false,
        }
    }

    fn start(&mut self) -> Result<Value, String> {
        // end error handler
        self.decide_next_data_type();
        Err(String::from("ff"))
    }

    fn decide_next_data_type(&mut self) -> Result<(), String> {
        match self.input[self.cursor] {
            b'd' => {
                self.cursor += 1;
                self.write_dict();
                Ok(())
            }
            b'l' => {
                self.cursor += 1;
                self.write_list();
                Ok(())
            }
            b'i' => {
                self.cursor += 1;
                self.write_int();
                Ok(())
            }
            b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                self.write_string();
                Ok(())
            }
            b'e' => {
                self.cursor += 1;
                Err(String::from(
                    "'e' where it shouldnt be (most likely a corrupted .torrent file)",
                ))
            }

            _ => Err(String::from(format!(
                "unknown charecter: {}",
                self.input[self.cursor]
            ))),
        }
    }

    fn write_dict(&mut self) {}
    fn write_list(&mut self) {}
    fn write_string(&mut self) {}
    fn write_int(&mut self) {}
}

//pub fn decode(input: &[u8]) -> Result<Torrent, String> {
//    Err(String::from("hello"))
//}
