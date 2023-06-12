use std::fs::File;
use std::io::prelude::*;





mod bencode;


use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() -> std::io::Result<()> {
    //let path = "test.torrent";
    //let mut file = File::open(path)?;
    //let mut buf = vec![];
    //file.read_to_end (&mut buf)?;
    //let contents = String::from_utf8_lossy (&buf);
    //println!("{:?}", contents);
    
    
    
    
    
    
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
    
    
    
    
    
    
    
    
    Ok(())
}