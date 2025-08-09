use serde_json::Value;

pub enum Input<'a> {
    Str(&'a str),
    Json(Value),
}

pub fn encode(input: Input) -> Result<Vec<u8>, String> {
    let json_object = match input {
        Input::Str(s) => serde_json::from_str(s).map_err(|e| e.to_string())?,
        Input::Json(v) => v,
    };
    Ok(get_values(json_object).unwrap())
}

fn get_values(val: Value) -> Result<Vec<u8>, String> {
    match val {
        // here add i.......e
        // check what type of number is accepted in bencode
        Value::Number(number) => Ok(format!("i{}e", number).as_bytes().to_vec()),
        // also add the 4:xxxxe
        Value::String(string) => {
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
        Value::Array(list) => {
            let mut result: Vec<u8> = Vec::new();
            result.push(b'l');
            for element in list {
                result.extend(get_values(element).unwrap());
            }
            result.push(b'e');
            Ok(result)
        }
        // here add d.......e
        Value::Object(dict) => {
            let mut result: Vec<u8> = Vec::new();
            result.push(b'd');
            for (k, v) in dict {
                result.extend(format!("{}:{}", k.len(), k).as_bytes().to_vec());
                result.extend(get_values(v).unwrap());
            }
            result.push(b'e');
            Ok(result)
        }
        _ => return Err(String::from("this should not happen")),
    }
}

// add tests
