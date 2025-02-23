use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Read;
use std::io::SeekFrom;
use std::os::unix::fs::FileExt;
use std::path::Path;

use crate::log::warning;
use crate::log::{debug, error, info};
use crate::torrent::Torrent;

//todo:  needs cleaning up, too many calculations they need to be organized in variables
pub(crate) fn write_single_file(torrent: &Torrent, index: u32, buf: Vec<u8>) -> Result<(), String> {
    let ind = index as u64;

    if Path::new(&torrent.info.name).exists() {
        info("file exists".to_string())
    } else {
        info("file does not exists. creating . . .".to_string());
        let test = File::create(&torrent.info.name).unwrap();
    }

    let file = File::options()
        .read(true)
        .write(true)
        .open(&torrent.info.name)
        .map_err(|e| e.to_string())?;

    /*
        ? check if file already has the bytes inited
        ? if for example the file is empty and we want to write piece 2 at index 262144
        ? it would cause an error
        ? must init all pieces with zeros before writing piece 2
    */

    //debug(format!("{}", file.metadata().unwrap().len()));
    //debug(format!("{}", ind));
    let file_len = file.metadata().unwrap().len();
    debug(format!(
        "before writing a piece: file len: {:?}, ind*piece_size: {:?}",
        file_len,
        ind * torrent.info.piece_length
    ));
    debug(format!("this piece length is: {}", buf.len()));

    if file_len < (ind * torrent.info.piece_length) {
        warning("here we add 00000....00000".to_string());
        // here write until ind with zeros
        let num_blocks_to_fill = ind - (file_len / torrent.info.piece_length);

        warning(format!(
            "we add {} blocks of size {}",
            num_blocks_to_fill, torrent.info.piece_length
        ));

        // write (num_blocks_to_fill*torrent.info.piece_length) with zeros
        let zeros: Vec<u8> = vec![0; (num_blocks_to_fill * torrent.info.piece_length) as usize];

        let test = file.write_at(&zeros, file_len).map_err(|e| e.to_string())?;
    }

    //debug(format!("{:?}", buf));
    //debug(format!("{}", ind));
    error(format!("{:?}", &buf.len()));
    let res = file.write_at(&buf, ind * torrent.info.piece_length);

    let file_len = file.metadata().unwrap().len();
    debug(format!(
        "after writing a piece: file len: {:?}, ind*piece_size: {:?}",
        file_len,
        ind * torrent.info.piece_length
    ));
    Ok(())
}

// todo: remove all prints and useless stuff
// todo: add ability to download pieces to text files and test with them because they are too large to put in github
#[cfg(test)]
mod tests {
    use std::fs;

    use crate::torrent;

    use super::*;
    #[test]
    fn it_works() {
        let torrent = Torrent {
            info: torrent::TorrentInfo {
                name: "POLEN23E.zip".to_string(),
                piece_length: 4190208,
                pieces: Default::default(),
                files: torrent::FileInfo::Single(34561190),
            },
            announce: Default::default(),
            announce_list: Default::default(),
            comment: Default::default(),
            creation_date: Default::default(),
            created_by: Default::default(),
            info_hash: Default::default(),
            peer_id: Default::default(),
        };

        let mut files = [
            (File::open("piece_8.txt").unwrap(), 8),
            (File::open("piece_4.txt").unwrap(), 4),
            (File::open("piece_7.txt").unwrap(), 7),
            (File::open("piece_3.txt").unwrap(), 3),
            (File::open("piece_6.txt").unwrap(), 6),
            (File::open("piece_0.txt").unwrap(), 0),
            (File::open("piece_1.txt").unwrap(), 1),
            (File::open("piece_2.txt").unwrap(), 2),
            (File::open("piece_5.txt").unwrap(), 5),
        ];

        for ind in 0..files.len() {
            let file_ind = files[ind].1;
            let mut file = &files[ind].0;

            let metadata = file.metadata().expect("unable to read metadata");

            info(format!(
                "in iteration: {ind}, for piece: {:?}, with length: {}, index: {}",
                fs::read_link(std::path::PathBuf::from(format!(
                    "/proc/self/fd/{}",
                    std::os::fd::AsRawFd::as_raw_fd(file)
                )))
                .unwrap(),
                metadata.len(),
                file_ind
            ));
            let mut buffer = vec![0; metadata.len() as usize];

            file.read(&mut buffer).expect("buffer overflow");
            let res = write_single_file(&torrent, file_ind.try_into().unwrap(), buffer);
            //std::thread::sleep(Duration::from_secs(10));
        }
    }
}
