use crate::bencode::decode;

use super::decode::DecoderElement;

pub struct Encoder{
    input: DecoderElement,
    cursor: usize,  // 0
    finished: bool, // 0
    start: Vec<u8>, // arrays to add letter to to combine later
    end: Vec<u8>, // arrays to add letter to to combine later
}

#[derive(Debug)]
pub struct EncodeError {
    message: String,
}





impl Encoder {
    pub fn new(input: DecoderElement) -> Self {
        Encoder {
            input,
            cursor: 0,
            finished: false,
            start: Vec::new(),
            end: Vec::new(),
        }
    }

    pub fn start(&mut self) -> Result<String, EncodeError>{
        match self.input{
            DecoderElement::Dict(_) => {
                Ok(String::from("result here"))
            },
            _ => {
                return Err(EncodeError { message: String::from("hello there") })
            }
        }
    }

    pub fn get_list(&mut self) -> std::io::Result<()>  {
        Ok(())
    }

    pub fn get_dict(&mut self) -> std::io::Result<()>  {
        Ok(())
    }


    pub fn get_string(&mut self) -> std::io::Result<()>  {
        Ok(())
    }

    // validation needs more work
    pub fn get_number(&mut self) -> std::io::Result<()>  {
        Ok(())
    }
}

