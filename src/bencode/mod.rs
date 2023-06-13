mod encode;
mod decode;


use decode::Decoder;


// set wheter it return string or byte arrays
// return something idk what yet
pub fn to_bencode(input: &[u8]) {
    let mut decoder = Decoder::new(input);
    let _ = decoder.start();
}





// return something idk what yet
pub fn from_bencode(input: &[u8]) {

}