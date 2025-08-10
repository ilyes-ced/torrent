use std::collections::HashMap;

#[derive(Debug)]
pub enum JsonObj {
    Dict(HashMap<String, JsonObj>),
    List(Vec<JsonObj>),
    String(String),
    Number(i64),
    Binary(Vec<u8>),
}

pub fn encode(val: JsonObj) -> Result<Vec<u8>, String> {
    match val {
        // here add i.......e
        // check what type of number is accepted in bencode
        JsonObj::Number(number) => Ok(format!("i{}e", number).as_bytes().to_vec()),
        // also add the 4:xxxxe
        JsonObj::String(string) => {
            let is_hex = string
                .split_whitespace()
                .all(|s| s.len() == 2 && u8::from_str_radix(s, 16).is_ok());

            // string can be normal text or binary data
            if is_hex {
                let bytes = string
                    .split_whitespace()
                    .map(|s| u8::from_str_radix(s, 16).map_err(|e| e.to_string()))
                    .collect::<Result<Vec<u8>, String>>()?;

                let mut result = format!("{}:", bytes.len()).as_bytes().to_vec();
                result.extend(bytes);
                Ok(result)
            } else {
                Ok(format!("{}:{}", string.len(), string).as_bytes().to_vec())
            }
        }
        // here add l.......e
        JsonObj::List(list) => {
            let mut result: Vec<u8> = Vec::new();
            result.push(b'l');
            for element in list {
                result.extend(encode(element).unwrap());
            }
            result.push(b'e');
            Ok(result)
        }
        // here add d.......e
        JsonObj::Dict(dict) => {
            let mut result: Vec<u8> = Vec::new();
            result.push(b'd');
            for (k, v) in dict {
                result.extend(format!("{}:{}", k.len(), k).as_bytes().to_vec());
                result.extend(encode(v).unwrap());
            }
            result.push(b'e');
            Ok(result)
        }
        // for binary things like NodeId
        JsonObj::Binary(vec) => {
            let mut result = format!("{}:", vec.len()).as_bytes().to_vec();
            result.extend(vec);
            Ok(result)
        }
        _ => return Err(String::from("this should not happen")),
    }
}

// add tests
