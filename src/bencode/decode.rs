use crate::bencode::decode;

pub struct Decoder<'a> {
    input: &'a [u8],
    cursor: usize,  // 0
    finished: bool, // 0
}

#[derive(Debug)]
pub struct DecodeError {
    message: String,
}

#[derive(Debug, Clone)]
pub struct Pair {
    pub name: String,
    pub value: DecoderElement,
}

#[derive(Debug, Clone)]
pub enum DecoderElement {
    Dict(Vec<Pair>),
    List(Vec<DecoderElement>),
    String(Vec<u8>),
    Number(Vec<u8>),
}

impl<'ser> Decoder<'ser> {
    pub fn new(input: &'ser [u8]) -> Self {
        Decoder {
            input,
            cursor: 0,
            finished: false,
        }
    }

    pub fn start(&mut self) -> Result<DecoderElement, DecodeError> {
        let result = match self.input[self.cursor] {
            b'd' => {
                self.cursor += 1;
                self.get_dict()
            }
            b'l' => {
                self.cursor += 1;
                self.get_list()
            }
            b'i' => {
                self.cursor += 1;
                self.get_number()
            }
            b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                self.get_string()
            }
            b'e' => {
                self.cursor += 1;
                Err(DecodeError {
                    message: String::from("error idk"),
                })
            }
       
            _ => Err(DecodeError {
                message: String::from(format!("file doesnt start with dict index: {}", self.input[self.cursor])),
            }),
        };

        Ok(result?)
    }

    pub fn get_list(&mut self) -> Result<DecoderElement, DecodeError> {
        let mut test: Vec<DecoderElement> = Vec::new();

        loop {
            let value: DecoderElement = match self.input[self.cursor] {
                b'd' => {
                    self.cursor += 1;
                    self.get_dict()?
                }
                b'l' => {
                    self.cursor += 1;
                    self.get_list()?
                }
                b'i' => {
                    self.cursor += 1;
                    self.get_number()?
                }
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    self.get_string()?
                }
                b'e' => {
                    self.cursor += 1;
                    break;
                }
                _ => {
                    return Err(DecodeError {
                        message: String::from(format!(
                            "unkknown token in get_dict: {} ---> index: {}",
                            self.input[self.cursor], self.cursor
                        )),
                    })
                }
            };
            test.push(value);
        }

        Ok(DecoderElement::List(test))
    }

    pub fn get_dict(&mut self) -> Result<DecoderElement, DecodeError> {
        let mut test: Vec<Pair> = Vec::new();

        loop {
            let string = match self.input[self.cursor] {
                b'e' => {
                    self.cursor += 1;
                    break;
                }
                _ => self.get_string()?,
            };
            let pair_value: DecoderElement = match self.input[self.cursor] {
                b'd' => {
                    self.cursor += 1;
                    self.get_dict()?
                }
                b'l' => {
                    self.cursor += 1;
                    self.get_list()?
                }
                b'i' => {
                    self.cursor += 1;
                    self.get_number()?
                }
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    self.get_string()?
                }
                _ => {
                    return Err(DecodeError {
                        message: String::from(format!(
                            "unkknown token in get_dict: {} ---> index: {}",
                            self.input[self.cursor], self.cursor
                        )),
                    })
                }
            };
            match string {
                DecoderElement::String(str) => test.push(Pair {
                    name: String::from_utf8_lossy(&str).to_string(),
                    value: pair_value,
                }),
                _ => {
                    return Err(DecodeError {
                        message: String::from(format!("the pair name attribute isnt of type string index: {}", self.cursor)),
                    })
                }
            }
        }

        Ok(DecoderElement::Dict(test))
    }

    pub fn get_string_len(&mut self) -> Result<usize, DecodeError> {
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
                    return Err(DecodeError {
                        message: String::from("dont know"),
                    })
                }
            }
        }

        Ok(concat(&value))
    }

    pub fn get_string(&mut self) -> Result<DecoderElement, DecodeError> {
        let lenght = self.get_string_len();
        let mut string: Vec<u8> = Vec::new();
        match lenght {
            Ok(len) => {
                for _ in 0..len {
                    string.push(self.input[self.cursor]);
                    self.cursor += 1;
                }
                Ok(DecoderElement::String(string))
            }
            Err(err) => return Err(err),
        }
    }

    pub fn get_number(&mut self) -> Result<DecoderElement, DecodeError> {
        let mut value: Vec<u8> = Vec::new();
        match self.input[self.cursor] {
            b'-' => {
                value.push(self.input[self.cursor]);
                self.cursor += 1;
            }
            b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8'
            | b'9' => {
                value.push(self.input[self.cursor]);
                self.cursor += 1;
            }
            _ => {
                return Err(DecodeError {
                    message: String::from("invalid at the start"),
                })
            }
        }
        loop {
            match self.input[self.cursor] {
                b'.' | b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8'
                | b'9' => {
                    value.push(self.input[self.cursor]);
                    self.cursor += 1;
                }
                b'e' => {
                    self.cursor += 1;
                    break;
                }
                _ => {
                    return Err(DecodeError {
                        message: String::from(format!("the pair name attribute isnt of type string index: {}", self.cursor)),
                    })
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
            //err
            return Err(DecodeError {
                message: String::from("invalid float format"),
            });
        }
        Ok(DecoderElement::Number(value))
    }
}

fn concat(vec: &Vec<u8>) -> usize {
    let mut acc: usize = 0;
    for elem in vec {
        acc *= 10;
        match elem {
            b'0' => acc += 0,
            b'1' => acc += 1,
            b'2' => acc += 2,
            b'3' => acc += 3,
            b'4' => acc += 4,
            b'5' => acc += 5,
            b'6' => acc += 6,
            b'7' => acc += 7,
            b'8' => acc += 8,
            b'9' => acc += 9,
            _ => {
                // impossible i think
            }
        }
    }
    acc
}
