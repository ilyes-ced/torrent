use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Read;
use std::io::SeekFrom;
use std::os::unix::fs::FileExt;
use std::path::Path;

use crate::log::{debug, error, info};
use crate::torrent::Torrent;

pub(crate) fn write_single_file(torrent: &Torrent, index: u32, buf: Vec<u8>) -> Result<(), String> {
    let ind = index as u64;

    debug("test000000000000000000".to_string());
    if Path::new(&torrent.info.name).exists() {
        info("file exists".to_string())
    } else {
        info("file does not exists. creating . . .".to_string());
        let test = File::create(&torrent.info.name).unwrap();
    }
    debug("test111111111111111111".to_string());

    let file = match File::options()
        .read(true)
        .write(true)
        .open(&torrent.info.name)
    {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };

    debug("test222222222222222222".to_string());

    /*
        ? check if file already has the bytes inited
        ? if for example the file is empty and we want to write piece 2 at index 262144
        ? it would cause an error
        ? must init all pieces with zeros before writing piece 2
    */

    debug("test333333333333333333".to_string());
    debug(format!("{}", file.metadata().unwrap().len()));
    debug(format!("{}", ind));
    if file.metadata().unwrap().len() < ind {
        // here write until ind with zeros
        let num_blocks_to_fill =
            (ind + 1) - (file.metadata().unwrap().len() / torrent.info.piece_length);

        debug("test444444444444444444".to_string());
        // write (num_blocks_to_fill*torrent.info.piece_length) with zeros
        let zeros: Vec<u8> = vec![
            0;
            (num_blocks_to_fill * torrent.info.piece_length)
                .try_into()
                .unwrap()
        ];

        debug("test555555555555555555".to_string());
        let test = file
            .write_at(&zeros, file.metadata().unwrap().len())
            .unwrap();
    }
    debug("test666666666666666666".to_string());
    debug(format!("{:?}", buf));
    debug(format!("{}", ind));
    let res = file.write_at(&buf, ind);

    debug("test777777777777777777".to_string());
    Ok(())
}
