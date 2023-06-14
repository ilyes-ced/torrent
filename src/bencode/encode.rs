use crate::bencode::decode;

pub struct Encoder<'a> {
    input: &'a [u8],
    cursor: usize,  // 0
    finished: bool, // 0
}

#[derive(Debug)]
pub struct EncodeError {
    message: String,
}

#[derive(Debug)]
struct Pair {
    name: String,
    value: EncoderElement,
}

#[derive(Debug)]
pub enum EncoderElement {
    Dict(Vec<Pair>),
    List(Vec<EncoderElement>),
    String(Vec<u8>),
    Number(Vec<u8>),
}

impl<'ser> Encoder<'ser> {
    pub fn new(input: &'ser [u8]) -> Self {
        Encoder {
            input,
            cursor: 0,
            finished: false,
        }
    }

    pub fn start(&mut self) -> Result<EncoderElement, EncodeError> {
        let result = match self.input[self.cursor] {
            b'd' => {
                self.cursor += 1;
                self.get_dict()
            }
            _ => Err(EncodeError {
                message: String::from("hello there"),
            }),
        };
        println!("started loop, {:#?}", result);

        Ok(result?)
    }

    pub fn get_list(&mut self) -> Result<EncoderElement, EncodeError> {
        let mut test: Vec<EncoderElement> = Vec::new();

        loop {
            let value: EncoderElement = match self.input[self.cursor] {
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
                    return Err(EncodeError {
                        message: String::from(format!(
                            "unkknown token in get_dict: {} ---> index: {}",
                            self.input[self.cursor], self.cursor
                        )),
                    })
                }
            };
            test.push(value);
        }

        Ok(EncoderElement::List(test))
    }

    pub fn get_dict(&mut self) -> Result<EncoderElement, EncodeError> {
        let mut test: Vec<Pair> = Vec::new();

        loop {
            let string = match self.input[self.cursor] {
                b'e' => {
                    self.cursor += 1;
                    break;
                }
                _ => self.get_string()?,
            };
            let pair_value: EncoderElement = match self.input[self.cursor] {
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
                    return Err(EncodeError {
                        message: String::from(format!(
                            "unkknown token in get_dict: {} ---> index: {}",
                            self.input[self.cursor], self.cursor
                        )),
                    })
                }
            };
            match string {
                EncoderElement::String(str) => test.push(Pair {
                    name: String::from_utf8_lossy(&str).to_string(),
                    value: pair_value,
                }),
                _ => {
                    return Err(EncodeError {
                        message: String::from("hello there"),
                    })
                }
            }
        }

        Ok(EncoderElement::Dict(test))
    }

    pub fn get_string_len(&mut self) -> Result<usize, EncodeError> {
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
                    // return error
                }
            }
        }

        Ok(concat(&value))
    }

    pub fn get_string(&mut self) -> Result<EncoderElement, EncodeError> {
        let lenght = self.get_string_len();
        let mut string: Vec<u8> = Vec::new();
        match lenght {
            Ok(len) => {
                for _ in 0..len {
                    string.push(self.input[self.cursor]);
                    self.cursor += 1;
                }
                //self.cursor -= 1;
                //Ok(EncoderElement::String(string))
                Ok(EncoderElement::String(string))
            }
            Err(err) => return Err(err),
        }
    }

    // validation needs more work
    pub fn get_number(&mut self) -> Result<EncoderElement, EncodeError> {
        let mut value: Vec<u8> = Vec::new();
        loop {
            match self.input[self.cursor] {
                b'-' | b'.' | b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8'
                | b'9' => {
                    value.push(self.input[self.cursor]);
                    self.cursor += 1;
                }
                b'e' => {
                    // jumps e
                    self.cursor += 1;
                    break;
                }
                _ => {
                    // return error
                }
            }
        }
        // validate
        let mut hyphen_counter = 0;
        let mut point_counter = 0;
        for char in &value {
            if char == &b'-' {
                hyphen_counter += 1
            } else if char == &b'.' {
                point_counter += 1
            }
        }
        if hyphen_counter > 1 || point_counter > 1 {
            //err
            return Err(EncodeError {
                message: String::from("invalid float format"),
            });
        }
        Ok(EncoderElement::Number(value))
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
                //err
            }
        }
    }
    acc.into()
}
