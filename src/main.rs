use std::fs::File;
use std::io::prelude::*;





mod bencode;



fn main() -> std::io::Result<()> {
    let path = "test.torrent";
    let mut file = File::open(path)?;
    let mut buf = vec![];
    file.read_to_end (&mut buf)?;
    //let contents = String::from_utf8_lossy (&buf);
    //println!("{:?}", contents);
    //println!("{:?}", buf);
    println!("{:?}", buf.len());
    
    bencode::to_bencode(&buf);
    
    

    
    
    
    Ok(())
}