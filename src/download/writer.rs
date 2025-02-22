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
        "before writing a piece: {:?},{:?}",
        file_len,
        ind * torrent.info.piece_length
    ));
    debug(format!("this piece length is: {}", buf.len()));

    if file_len < (ind * torrent.info.piece_length) {
        warning("here we add 00000....00000".to_string());
        // here write until ind with zeros
        let num_blocks_to_fill = (ind + 1) - (file_len / torrent.info.piece_length);

        // write (num_blocks_to_fill*torrent.info.piece_length) with zeros
        let zeros: Vec<u8> = vec![0; (num_blocks_to_fill * torrent.info.piece_length) as usize];

        let test = file.write_at(&zeros, file_len).map_err(|e| e.to_string())?;
    }

    //debug(format!("{:?}", buf));
    //debug(format!("{}", ind));
    let res = file.write_at(&buf, (ind * torrent.info.piece_length));

    let file_len = file.metadata().unwrap().len();
    debug(format!(
        "after writing a piece: {:?},{:?}",
        file_len,
        ind * torrent.info.piece_length
    ));
    Ok(())
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use crate::torrent::{FileInfo, TorrentInfo};

    use super::*;
    use std::fs::{self, OpenOptions};

    #[test]
    fn test_file_exists() {
        let temp_file = NamedTempFile::new().unwrap();
        let torrent = Torrent {
            info: TorrentInfo {
                name: temp_file.path().to_str().unwrap().to_string(),
                piece_length: 16,
                pieces: Default::default(),
                files: FileInfo::Single(20),
            },
            announce: Default::default(),
            announce_list: Default::default(),
            comment: Default::default(),
            creation_date: Default::default(),
            created_by: Default::default(),
            info_hash: Default::default(),
            peer_id: Default::default(),
        };

        let buf = vec![1, 2, 3, 4];
        let result = write_single_file(&torrent, 0, buf.clone());
        assert!(result.is_ok());

        let mut file = File::open(temp_file.path()).unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        assert_eq!(contents, buf);
    }

    #[test]
    fn test_file_does_not_exist() {
        let temp_file = NamedTempFile::new().unwrap();
        let torrent = Torrent {
            info: TorrentInfo {
                name: temp_file.path().to_str().unwrap().to_string(),
                piece_length: 16,
                pieces: Default::default(),
                files: FileInfo::Single(20),
            },
            announce: Default::default(),
            announce_list: Default::default(),
            comment: Default::default(),
            creation_date: Default::default(),
            created_by: Default::default(),
            info_hash: Default::default(),
            peer_id: Default::default(),
        };

        // Remove the temp file to simulate it not existing
        fs::remove_file(temp_file.path()).unwrap();

        let buf = vec![1, 2, 3, 4];
        let result = write_single_file(&torrent, 0, buf.clone());
        assert!(result.is_ok());

        let mut file = File::open(temp_file.path()).unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        assert_eq!(contents, buf);
    }

    #[test]
    fn test_file_smaller_than_index() {
        let temp_file = NamedTempFile::new().unwrap();
        let torrent = Torrent {
            info: TorrentInfo {
                name: temp_file.path().to_str().unwrap().to_string(),
                piece_length: 16,
                pieces: Default::default(),
                files: FileInfo::Single(20),
            },
            announce: Default::default(),
            announce_list: Default::default(),
            comment: Default::default(),
            creation_date: Default::default(),
            created_by: Default::default(),
            info_hash: Default::default(),
            peer_id: Default::default(),
        };

        let buf = vec![1, 2, 3, 4];
        let result = write_single_file(&torrent, 32, buf.clone());
        assert!(result.is_ok());

        let mut file = File::open(temp_file.path()).unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        assert_eq!(contents.len(), 36);
        assert_eq!(&contents[32..36], &buf);
    }

    #[test]
    fn test_file_large_enough() {
        let temp_file = NamedTempFile::new().unwrap();
        let torrent = Torrent {
            info: TorrentInfo {
                name: temp_file.path().to_str().unwrap().to_string(),
                piece_length: 16,
                pieces: Default::default(),
                files: FileInfo::Single(20),
            },
            announce: Default::default(),
            announce_list: Default::default(),
            comment: Default::default(),
            creation_date: Default::default(),
            created_by: Default::default(),
            info_hash: Default::default(),
            peer_id: Default::default(),
        };

        // Write initial data to make the file large enough
        let mut file = OpenOptions::new()
            .write(true)
            .open(temp_file.path())
            .unwrap();
        file.write_all(&vec![0; 64]).unwrap();

        let buf = vec![1, 2, 3, 4];
        let result = write_single_file(&torrent, 32, buf.clone());
        assert!(result.is_ok());

        let mut file = File::open(temp_file.path()).unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        assert_eq!(contents.len(), 64);
        assert_eq!(&contents[32..36], &buf);
    }
}
