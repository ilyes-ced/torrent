use std::fs::File;

use std::io::prelude::*;


use crate::bencode::decode;

use super::decode::{DecoderElement, Pair};



#[derive(Debug)]
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

    pub fn start(&mut self) -> Result<Vec<u8>, EncodeError>{
        let decoded_element = self.input.clone();
        match decoded_element {
            DecoderElement::Dict(element) => {
                self.write_dict_pairs(element)?;
            },
            _ => {
                //err
            },
        }


        println!("{:?}", self);


        self.start.append(&mut self.end);
        println!("{:?}", self.start);
        println!("{:?}", String::from_utf8_lossy(&self.start).to_string());
        
        let mut file = File::create("output.txt").unwrap();
        write!(file, "{:?}", self.start);
        write!(file, "{}", String::from_utf8_lossy(&self.start).to_string());


        Ok(self.start.clone())
            
    }

    pub fn write_dict_pairs(&mut self, elements: Vec<Pair>) -> Result<(), EncodeError>  {
        self.start.push(b'd');
        for element in elements {
            let gg = usize::to_be_bytes(element.name.len());
            for n in gg{
                if n != 0 {
                    println!("{:x?}", n);
                    self.start.push(n);
                }
            }
            self.start.push(b':');
            // not tested
            match element.value {
                DecoderElement::Dict(ele) => self.write_dict_pairs(ele)?,
                DecoderElement::List(ele) => self.write_list(ele)?,
                DecoderElement::String(ele) => self.write_string(ele)?,
                DecoderElement::Number(ele) => self.write_number(ele)?,
            };
        }
        self.end.push(b'e');
        Ok(())
    }



    pub fn write_list(&mut self, elements: Vec<DecoderElement>) -> Result<(), EncodeError>  {
        self.start.push(b'l');
        for element in elements {
            match element {
                DecoderElement::Dict(ele) => self.write_dict_pairs(ele)?,
                DecoderElement::List(ele) => self.write_list(ele)?,
                DecoderElement::String(ele) => self.write_string(ele)?,
                DecoderElement::Number(ele) => self.write_number(ele)?,
            };
        }
        self.end.push(b'e');
        Ok(())
    }

    pub fn write_string(&mut self, elements: Vec<u8>) -> Result<(), EncodeError>  {
        self.start.extend_from_slice(elements.as_slice()); 
        Ok(())
    }

    // validation needs more work
    pub fn write_number(&mut self, elements: Vec<u8>) -> Result<(), EncodeError>  {
        self.start.extend_from_slice(elements.as_slice()); 
        Ok(())
    }
}

