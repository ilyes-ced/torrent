use std::fs::File;
use std::io::prelude::*;



fn main() -> std::io::Result<()> {
    //let mut file = File::open("test.torrent")?;
    //let mut contents = String::new();
    //file.read_to_string(&mut contents)?;
    //println!("{:?}", contents);


    //println!("{:?}", file.file_name());
    let path = "test.torrent";

    let mut file = File::open(path)?;
    let mut buf = vec![];
    file.read_to_end (&mut buf)?;
    let contents = String::from_utf8_lossy (&buf);
    println!("{:?}", contents);
    Ok(())
}