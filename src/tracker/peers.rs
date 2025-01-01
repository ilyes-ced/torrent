use crate::constants;
use crate::torrent::Torrent;
use rand::distributions::{Alphanumeric, DistString};
use reqwest::{blocking::Client, Error};

pub struct Peers {}

impl Peers {
    pub fn get_peers(torrent_data: Torrent) -> Result<Peers, String> {
        println!("---------------------");
        println!("---------------------");
        println!("---------------------");
        println!("---------------------");
        let url = build_http_url(torrent_data).unwrap();
        println!("{}", url);
        let result = send_request(url);
        println!("{}", result.unwrap());

        Ok(Peers {})
    }
}

fn send_request(url: String) -> Result<String, Error> {
    let client = Client::new();

    let response = client.get(url).send()?;

    if response.status().is_success() {
        let body = response.text()?;
        println!("Response Body:\n{}", body);
    } else {
        println!("Failed to fetch data: {}", response.status());
    }

    Ok(String::new())
}

fn build_http_url(torrent_data: Torrent) -> Result<String, String> {
    let url = torrent_data.announce
        + "?info_hash="
        //f0,38,41,b6,b2,45,85,b1,57,1d,f6,b0,c9,30,d9,94,60,47,05,8f
        //[240, 56, 65, 182, 178, 69, 133, 177, 87, 29, 246, 176, 201, 48, 217, 148, 96, 71, 5, 143]
        //+ &encode_bin([240, 56, 65, 182, 178, 69, 133, 177, 87, 29, 246, 176, 201, 48, 217, 148, 96, 71, 5, 143])
        + &encode_bin(torrent_data.info_hash)
        + "&peer_id="
        + &encode_bin(new_peer_id())
        + "&port="
        + &constants::PORT.to_string()
        + "&uploaded="
        + "0" //uploaded
        + "&downloaded="
        + "0" //downloaded
        + "&left="
        + "0"; //left calculate it later

    Ok(url)
}

fn new_peer_id() -> [u8; 20] {
    //"-IT0001-"+12 random chars
    let mut id = [0; 20];
    id[0..8].copy_from_slice("-IT0001-".as_bytes());
    let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 12);
    id[8..20].copy_from_slice(string.as_bytes());
    id
}

fn encode_bin(input: [u8; 20]) -> String {
    let mut return_string = String::new();
    for byte in input {
        return_string.push_str("%");
        return_string.push_str(&format!("{:02x}", byte));
    }
    return_string
}
