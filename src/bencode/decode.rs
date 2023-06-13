use crate::bencode::decode;

pub struct Decoder<'a> {
    input: &'a [u8],
    cursor: usize, // 0
    finished: bool, // 0
}

#[derive(Debug)]
pub struct DecodeError {
    message: String
}




#[derive(Debug)]
struct Pair {
    name: String,
    value: Element
}
#[derive(Debug)]
pub enum Element {
    Dict(Vec<Pair>),
    List(Vec<Element>),
    String(Vec<u8>),
    Number(Vec<u8>)
}


impl<'ser> Decoder<'ser>  {
    pub fn new(input: &'ser [u8]) -> Self {
        Decoder{
            input,
            cursor: 0,
            finished: false,
        }
    }







    pub fn start(&mut self) -> Result<Element, DecodeError> {
        // here we create mut object and add all results
        //while !self.finished {
        //    match self.input[self.cursor] {
        //        b'd' => {
        //            self.cursor += 1;
        //            self.get_dict();
        //        },
        //        b'l' => {
        //            self.cursor += 1;
        //            self.get_list();
        //        },
        //        
        //        // unllikely to happen here for string and numbers
        //        b'i' => {
        //            // fetch number
        //            self.get_number();
        //        },
        //        b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
        //            self.get_string();
        //        },
        //        _ => {
        //            // return error
        //        }
        //    }
        //    self.cursor += 1;
        //    if self.cursor == self.input.len() {
        //        self.finished = true
        //    }
        //}



        let result = match self.input[self.cursor] {
            b'd' => {
                self.cursor += 1;
                self.get_dict()
            },
            _ => {
                Err(DecodeError { message: String::from("hello there") })
            }
        }?;


        println!("started loop, {:?}", result);
        Ok(result)

        
    }











    pub fn get_list(&mut self) -> Result<Element, DecodeError> {
        let mut test: Vec<Element> = Vec::new();

        loop {
            println!("started loop, {:?}", self.input[self.cursor]);
            println!("started loop, {:?}", self.cursor);
            println!("started loop, {:?}", test);
            let value: Element = match self.input[self.cursor] {
                b'd' => {
                    self.cursor += 1;
                    self.get_dict()?
                },
                b'l' => {
                    self.cursor += 1;
                    self.get_list()?
                },
                b'i' => {
                    self.cursor += 1;
                    self.get_number()?
                },
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    self.get_string()?
                },
                b'e' => break,
                _ => {
                    return Err(DecodeError{message: String::from(format!("unkknown token in get_dict: {} ---> index: {}", self.input[self.cursor], self.cursor))})
                }
            };
            test.push(value);
        }
    
        Ok(Element::List(test))
    }





























    pub fn get_dict(&mut self) -> Result<Element, DecodeError> {
        let mut test: Vec<Pair> = Vec::new();

        loop {
            let string = match self.input[self.cursor] {
                b'e' => break,
                _ => self.get_string()?
            };
            let pair_value: Element = match self.input[self.cursor] {
                b'd' => {
                    self.cursor += 1;
                    self.get_dict()?
                },
                b'l' => {
                    self.cursor += 1;
                    println!("started lis gtetting, {:?}", self.input[self.cursor]);
                    println!("started lis gtetting, {:?}", self.cursor);
                    self.get_list()?
                },
                b'i' => {
                    self.cursor += 1;
                    self.get_number()?
                },
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    self.get_string()?
                },
                _ => {
                    return Err(DecodeError{message: String::from(format!("unkknown token in get_dict: {} ---> index: {}", self.input[self.cursor], self.cursor))})
                }
            };
            match string {
                Element::String(str) => test.push(Pair { name: String::from_utf8_lossy(&str).to_string(), value: pair_value }),
                _ => return Err(DecodeError { message: String::from("hello there") })
            }
        }
    
        Ok(Element::Dict(test))
    
    }















































    pub fn get_string_len(&mut self) -> Result<usize, DecodeError> {
        let mut value: Vec<u8> = Vec::new();
        loop {
            match self.input[self.cursor] {
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    value.push(self.input[self.cursor]);
                    self.cursor += 1;
                },
                b':' => {
                    // jumps :
                    self.cursor += 1;
                    break
                },
                _ => {
                    // return error
                }
            }
        }

        Ok(concat(&value))
    }
    




    pub fn get_string(&mut self) -> Result<Element, DecodeError> {
        let lenght = self.get_string_len();
        let mut string: Vec<u8> = Vec::new();
        match lenght {
            Ok(len) => {
                for _ in 0..len{
                    string.push(self.input[self.cursor]);
                    self.cursor += 1;
                }
                //self.cursor -= 1;
                Ok(Element::String(string))
            },
            Err(err) => {
                return Err(err)
            }
        }
    }
    
    pub fn get_number(&mut self) -> Result<Element, DecodeError> {
        let mut value: Vec<u8> = Vec::new();
        println!("got umber , {:?}", value);
        println!("got umber , {:?}", value);
        loop {
            match self.input[self.cursor] {
                b'-' | b'.' | b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    value.push(self.input[self.cursor]);
                    self.cursor += 1;
                },
                b'e' => {
                    // jumps e
                    self.cursor += 1;
                    break
                },
                _ => {
                    // return error
                }
            }
        }
        println!("got umber , {:?}", value);

        Ok(Element::Number(value))
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
