use crate::utils::concat;
use sha1::{Digest, Sha1};

pub struct Decoder {
    input: Vec<u8>,
    info_hash: [u8; 20],
    cursor: usize,
    finished: bool,
}

#[derive(Debug)]
pub struct DecoderResults {
    pub info_hash: [u8; 20],
    pub result: String,
}

impl Decoder {
    pub fn new(input: &[u8]) -> Decoder {
        Decoder {
            input: input.to_vec(),
            info_hash: [0; 20],
            cursor: 0,
            finished: false,
        }
    }

    pub fn start(&mut self) -> Result<DecoderResults, String> {
        let mut result = String::new();
        loop {
            if self.finished == true || self.cursor == self.input.len() {
                break;
            } else {
                result = format!("{}{}", result, self.next().unwrap());
            }
        }
        Ok(DecoderResults {
            info_hash: self.info_hash,
            result,
        })
    }

    fn next(&mut self) -> Result<String, String> {
        match self.input[self.cursor] {
            b'd' => {
                self.cursor += 1;
                Ok(self.get_dict().unwrap())
            }
            b'l' => {
                self.cursor += 1;
                Ok(self.get_list().unwrap())
            }
            b'i' => {
                self.cursor += 1;
                Ok(self.get_int().unwrap())
            }
            b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                // puts the string value in ""
                Ok(format!("\"{}\"", self.get_string().unwrap()))
            }
            b' ' => {
                self.cursor += 1;
                self.next()
            }

            _ => Err(format!("unknown charecter: {}", self.input[self.cursor])),
        }
    }

    fn info_binary(&mut self) -> Result<Vec<u8>, String> {
        let mut info_bin: Vec<u8> = Vec::new();
        let mut e_counter = 0;
        'outer: loop {
            match self.input[self.cursor] {
                b'd' => {
                    info_bin.push(self.input[self.cursor]);
                    e_counter += 1;
                }
                b'l' => {
                    info_bin.push(self.input[self.cursor]);
                    e_counter += 1;
                }
                b'i' => {
                    info_bin.push(self.input[self.cursor]);
                    self.cursor += 1;

                    let int = self.get_int().unwrap();
                    let int_bytes = int.as_bytes().to_vec();

                    for i in int_bytes {
                        info_bin.push(i);
                    }
                    info_bin.push(b'e');
                    self.cursor -= 1;
                }
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    let str_len = self.get_string_len().unwrap();
                    let str_len_string = str_len.to_string();
                    let str_len_bytes = str_len_string.as_bytes();

                    for i in str_len_bytes {
                        info_bin.push(*i);
                    }
                    info_bin.push(b':');
                    for i in 0..str_len {
                        info_bin.push(self.input[self.cursor]);
                        if i != str_len - 1 {
                            self.cursor += 1;
                        }
                    }
                }
                b'e' => {
                    info_bin.push(self.input[self.cursor]);
                    e_counter -= 1;
                    if e_counter == 0 {
                        break 'outer;
                    }
                }

                _ => info_bin.push(self.input[self.cursor]),
            }
            self.cursor += 1;
        }
        // return cursor to original place
        // remove start d and end e
        //info_bin.remove(0);
        //info_bin.pop();
        Ok(info_bin)
    }

    fn get_dict(&mut self) -> Result<String, String> {
        let mut current_dict = String::from("{");
        loop {
            let name = match self.input[self.cursor] {
                b'e' => {
                    self.cursor += 1;
                    break;
                }
                _ => self.get_string().unwrap(),
            };

            if name == "info" {
                let original_cursor = self.cursor;
                let info_binary = self.info_binary().unwrap();

                // hashing
                let mut hasher = Sha1::new();
                hasher.update(info_binary.clone());
                let result = hasher.finalize();
                self.info_hash = result.into();

                self.cursor = original_cursor;
            }

            let value = self.next().unwrap();

            current_dict = format!("{}\"{}\":{},", current_dict, name, value)
        }
        // removes last ,
        current_dict.pop();
        current_dict = format!("{}}}", current_dict);

        Ok(current_dict)
    }

    fn get_list(&mut self) -> Result<String, String> {
        let mut current_list = String::from("[");
        loop {
            let value = match self.input[self.cursor] {
                b'e' => {
                    self.cursor += 1;
                    break;
                }
                _ => self.next().unwrap(),
            };
            current_list = format!("{}{},", current_list, value)
        }
        // removes last ,
        current_list.pop();
        current_list = format!("{}]", current_list);
        Ok(current_list)
    }

    pub fn get_string_len(&mut self) -> Result<usize, String> {
        let mut value: Vec<u8> = Vec::new();
        loop {
            match self.input[self.cursor] {
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    value.push(self.input[self.cursor]);
                    self.cursor += 1;
                }
                b':' => {
                    self.cursor += 1;
                    break;
                }
                _ => {
                    return Err(format!(
                        "dont know {}, {}",
                        self.cursor, self.input[self.cursor]
                    ))
                }
            }
        }

        Ok(concat(&value))
    }
    fn get_string(&mut self) -> Result<String, String> {
        let lenght = self.get_string_len();
        let mut string: Vec<u8> = Vec::new();
        match lenght {
            Ok(len) => {
                for _ in 0..len {
                    string.push(self.input[self.cursor]);
                    self.cursor += 1;
                }
                let value = match String::from_utf8(string.clone()) {
                    Ok(val) => val,
                    Err(_) => {
                        // this case is for the binary data in pieces
                        let collected: Vec<String> = string
                            .iter()
                            .map(|byte| format!("{:02X}", byte)) // Format each byte as two hex digits
                            .collect();
                        return Ok(collected.join(" "));
                    }
                };
                // "" inside rust strings can cause errors
                let value = value.replace("\"", "'");
                Ok(value)
            }
            Err(err) => return Err(err),
        }
    }
    fn get_int(&mut self) -> Result<String, String> {
        let mut value: Vec<u8> = Vec::new();
        match self.input[self.cursor] {
            b'-' => {
                value.push(self.input[self.cursor]);
                self.cursor += 1;
            }
            b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                value.push(self.input[self.cursor]);
                self.cursor += 1;
            }
            _ => {
                return Err(format!(
                    "invalid at the start, index: {}, value:{}",
                    self.cursor, self.input[self.cursor]
                ))
            }
        }
        loop {
            match self.input[self.cursor] {
                b'.' | b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    value.push(self.input[self.cursor]);
                    self.cursor += 1;
                }
                b'e' => {
                    self.cursor += 1;
                    break;
                }
                _ => {
                    return Err(format!(
                        "the pair name attribute isnt of type string index: {}",
                        self.cursor
                    ))
                }
            }
        }
        // validate
        let mut point_counter = 0;
        for char in &value {
            if char == &b'.' {
                point_counter += 1
            }
        }
        if point_counter > 1 {
            return Err(String::from("invalid float format"));
        }
        Ok(String::from_utf8(value).unwrap())
    }
}

// tests:
//  broken file
//  broken integer
//  broken signed number
//  broken float
//  broken list
//  broken dict
//  correct bencode decoding
//

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test1() {}
//}
