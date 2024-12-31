pub mod decode;

use decode::{Decoder, DecoderResults};

pub fn decode(input: &[u8]) -> Result<DecoderResults, String> {
    let result = Decoder::new(input).start().unwrap();
    Ok(result)
}
