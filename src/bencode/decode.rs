use crate::utils::concat;

pub struct Decoder {
    input: Vec<u8>,
    result: String,
    info_binary: Vec<u8>,
    cursor: usize,
    finished: bool,
}

impl Decoder {
    pub fn new(input: &[u8]) -> Decoder {
        Decoder {
            input: input.to_vec(),
            result: String::new(),
            info_binary: Vec::new(),
            cursor: 0,
            finished: false,
        }
    }

    pub fn start(&mut self) -> Result<String, String> {
        //check data is not empty
        let mut result = String::new();
        // end error handler
        loop {
            if self.finished == true || self.cursor == self.input.len() {
                break;
            } else {
                result = format!("{}{}", result, self.next().unwrap());
            }
        }
        Ok(result)
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

            _ => Err(String::from(format!(
                "unknown charecter: {}",
                self.input[self.cursor]
            ))),
        }
    }

    fn info_binary(&mut self) -> Result<Vec<u8>, String> {
        let mut info_bin: Vec<u8> = Vec::new();
        let mut e_counter = 0;
        'outer: loop {
            match self.input[self.cursor] {
                b'd' | b'l' => {
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
                        //self.cursor += 1;
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
                    self.cursor += 1;
                    e_counter -= 1;
                    if e_counter == 0 {
                        let _ = std::fs::write("output.txt", &info_bin);
                        break 'outer;
                    }
                }

                _ => info_bin.push(self.input[self.cursor]),
            }
            self.cursor += 1;
        }
        // return cursor to orginal place
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
                self.info_binary = self.info_binary().unwrap();
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
                    return Err(String::from(format!(
                        "dont know {}, {}",
                        self.cursor, self.input[self.cursor]
                    )))
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
                return Err(String::from(format!(
                    "invalid at the start, index: {}, value:{}",
                    self.cursor, self.input[self.cursor]
                )))
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
                    return Err(String::from(format!(
                        "the pair name attribute isnt of type string index: {}",
                        self.cursor
                    )))
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dict() {
        let bencode =
            "d6:string5:value6:numberi123e4:listl11:list-item-111:list-item-2ee".as_bytes();
        let mut bytes = Decoder::new(bencode);
        assert_eq!(
            bytes.start(),
            Ok(String::from(
                "{\"string\":\"value\",\"number\":123,\"list\":[\"list-item-1\",\"list-item-2\"]}"
            ))
        );
    }

    #[test]
    fn test_list() {
        let bencode = "li42e5:hellol3:foo3:bared4:name4:John3:agei30eee".as_bytes();
        let mut bytes = Decoder::new(bencode);
        assert_eq!(
            bytes.start(),
            Ok(String::from(
                "[42,\"hello\",[\"foo\",\"bar\"],{\"name\":\"John\",\"age\":30}]"
            ))
        );
    }
}
