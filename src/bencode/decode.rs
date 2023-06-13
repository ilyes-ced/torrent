use crate::bencode::decode;

pub struct Decoder<'a> {
    input: &'a [u8],
    cursor: usize, // 0
    finished: bool, // 0
}


pub struct DecodeError {
    message: String
}





struct Pair {
    name: String,
    value: Element
}
pub enum Element {
    Dict(Vec<Pair>),
    List(Vec<Element>),
    String(String),
    Number{positive: bool, value: usize}
}


impl<'ser> Decoder<'ser>  {
    pub fn new(input: &'ser [u8]) -> Self {
        Decoder{
            input,
            cursor: 0,
            finished: false,
        }
    }







    pub fn start(&mut self) {
        // here we create mut object and add all results
        while !self.finished {
            match self.input[self.cursor] {
                b'd' => {
                    self.cursor += 1;
                    self.get_dict();
                },
                b'l' => {
                    self.cursor += 1;
                    self.get_list();
                },

                
                // unllikely to happen here for string and numbers
                b'i' => {
                    // fetch number
                    self.get_number();
                },
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    self.get_string();
                },
                _ => {
                    // return error
                }
            }
            self.cursor += 1;
            if self.cursor == self.input.len() {
                self.finished = true
            }
        }
        println!("finished loop, {}", self.cursor);
    }











    pub fn get_list(&mut self) -> Result<Element, DecodeError> {
        let mut test: Vec<Element> = Vec::new();
        // first one to init the test variable

        loop {
            match self.input[self.cursor] {
                b'd' => {
                    test.push(self.get_dict()?)
                },
                b'l' => {
                    test.push(self.get_list()?)
                },
                b'i' => {
                    test.push(self.get_number()?)
                },
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    test.push(self.get_string()?)
                },
                b'e' => {
                    break;
                }
                _ => {
                    return Err(DecodeError{message: String::from(format!("unkknown toke: {}", self.input[self.cursor]))})
                }
            }
        }
        println!("finished loop, {}", self.cursor);
    
        Ok(Element::List(test))
    
    }





























    
    pub fn get_dict(&self) -> Result<Element, DecodeError> {
        let mut test: Vec<Pair> = Vec::new();
        // first one to init the test variable
        //d
        //n:string  OR   l.......e    OR     i...e      OR      d...

        loop {
            match self.input[self.cursor] {
                b'd' => {
                    test.push(self.get_dict()?)
                },
                b'l' => {
                    test.push(self.get_list()?)
                },
                b'i' => {
                    test.push(self.get_number()?)
                },
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    test.push(self.get_string()?)
                },
                b'e' => {
                    break;
                }
                _ => {
                    return Err(DecodeError{message: String::from(format!("unkknown toke: {}", self.input[self.cursor]))})
                }
            }
        }
        println!("finished loop, {}", self.cursor);
    
        Ok(Element::String(String::new()))
    
    }
















































    pub fn get_string_len(&mut self) -> Result<usize, DecodeError> {
        let mut value: Vec<u8> = Vec::new();
        loop {

            match self.input[self.cursor] {
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    value.push(self.input[self.cursor]);
                    println!("valueee*******{:?}", value);
                    println!("valueee*******{}", self.input[self.cursor]);
                    println!("leeeeen*******{}", self.cursor);
                    self.cursor += 1;
                },
                b':' => break,
                _ => {
                    println!("errrrrrrrrrrrrrrrrrrrrrrrrrrrr*******{:?}", value);
                    println!("errrrrrrrrrrrrrrrrrrrrrrrrrrrr*******{}", self.input[self.cursor]);
                    // return error
                }
            }
        }
        // +1 for :
        self.cursor += value.len()+1;
        // value as number
        println!("*******{}", concat(&value));
        Ok(concat(&value))
    }
    




    pub fn get_string(&mut self) -> Result<Element, DecodeError> {
        let lenght = self.get_string_len();
        match lenght {
            Ok(len) => {
                self.cursor += len + 1;
                Ok(Element::String(String::new()))
            },
            Err(err) => {
                return Err(err)
            }
        }
    }
    
    pub fn get_number(&self) -> Result<Element, DecodeError> {
        
        Ok(Element::String(String::new()))
    }
    
    

}





fn concat(vec: &Vec<u8>) -> usize {
    let mut acc: usize = 0;
    for elem in vec {
        acc *= 10;
        acc += *elem as usize;
    }
    acc.into()
}

















