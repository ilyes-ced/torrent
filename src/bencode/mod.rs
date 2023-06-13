mod encode;
mod decode;


use decode::Decoder;

// return something idk what yet
pub fn to_bencode(input: &[u8]) {
    let mut decoder = Decoder::new(input);
    decoder.start();
}





// return something idk what yet
pub fn from_bencode(input: &[u8]) {

}