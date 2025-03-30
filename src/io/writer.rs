use std::fs::{create_dir_all, File};
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};

use super::mapping::mapping;
use crate::download::download::PieceResult;
use crate::log::{debug, info};
use crate::torrentfile::torrent::{
    FileInfo::{Multiple, Single},
    {Files, Torrent},
};

//todo:  needs cleaning up, too many calculations they need to be organized in variables

pub(crate) fn write_file(
    torrent: &Torrent,
    piece: PieceResult,
    download_dir: String,
) -> Result<(), String> {
    match &torrent.info.files {
        Single(_) => write_single_file(torrent, piece, download_dir),
        Multiple(files) => write_multi_file(torrent, piece, files, download_dir),
    }
}

pub(crate) fn write_single_file(
    torrent: &Torrent,
    piece: PieceResult,
    download_dir: String,
) -> Result<(), String> {
    let ind = piece.index as u64;
    let path = PathBuf::from(download_dir).join(&torrent.info.name);

    let file = get_file(path)?;
    let piece_len = torrent.info.piece_length;

    /*
    ? this part was intended to write empty blocks of zeros when we download the last pieces before the first pieces so we fill the first ones with zeros but it turns out that .write_at() already does that by default
    */
    //if file_len < (ind * piece_len) {
    //    let num_blocks_to_fill = ind - (file_len / piece_len);
    //    warning(format!(
    //        "Adding {} blocks of size {}",
    //        num_blocks_to_fill, piece_len
    //    ));
    //    let zeros: Vec<u8> = vec![0; (num_blocks_to_fill * piece_len) as usize];
    //    file.write_at(&zeros, file_len).map_err(|e| e.to_string())?;
    //}

    file.write_at(&piece.buf, ind * piece_len)
        .map_err(|err| err.to_string())?;

    Ok(())
}

fn write_multi_file(
    torrent: &Torrent,
    piece: PieceResult,
    files: &[Files],
    download_dir: String,
) -> Result<(), String> {
    // we have files in torrent and piece index we can calculate to which file or multiple files each pioece belongs
    let mappings = mapping(torrent, piece.index)?;

    debug(format!(
        "for piece: {}, mappings {:?}",
        piece.index, mappings
    ));

    for (map_ind, mapping) in mappings.iter().enumerate() {
        let file_path = PathBuf::from(download_dir.clone())
            .join(&files[mapping.file_index].clone().paths.join("/"));

        let file = get_file(file_path)?;

        let piece_len = torrent.info.piece_length;

        let buffer = if mappings.len() == 1 {
            &piece.buf
        } else if mappings.len() == 2 {
            if map_ind == 0 {
                &piece.buf[0..mapping.piece_write_len as usize]
            } else {
                &piece.buf[(piece_len - mapping.piece_write_len) as usize..]
            }
        } else if mappings.len() > 2 {
            if map_ind == 0 {
                &piece.buf[mapping.piece_write_len as usize..]
            } else if map_ind == mappings.len() - 1 {
                &piece.buf[..mapping.piece_write_len as usize]
            } else {
                // todo: untested
                &piece.buf[mappings[map_ind - 1].piece_write_len as usize
                    ..mappings[map_ind + 1].piece_write_len as usize]
            }
        } else {
            return Err(String::from(
                "should never happen, that a piece belongs to no files",
            ));
        };

        file.write_at(buffer, mapping.file_write_offset)
            .map_err(|err| err.to_string())?;
    }
    println!("\n");
    Ok(())
}

fn get_file(path: PathBuf) -> Result<File, String> {
    if !Path::new(&path).exists() {
        info(format!(
            "file \" {:?} \" does not exists. creating . . .",
            path
        ));
        if let Some(parent) = path.parent() {
            create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        File::create(&path).unwrap();
    }

    let file = File::options()
        .read(true)
        .write(true)
        .open(path)
        .map_err(|e| e.to_string())?;

    Ok(file)
}

//////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////
//#[cfg(test)]
//mod tests {
//    use super::*;
//    use crate::torrentfile::torrent::{FileInfo, Torrent};
//    use std::fs::{remove_file, OpenOptions};
//    use std::path::Path;
//
//    fn mock_torrent_multiple_files() -> Torrent {
//        Torrent {
//            info: crate::torrentfile::torrent::Info {
//                piece_length: 256, // Example piece length
//                pieces: vec![],    // Mock, as we don't need actual pieces for the test
//                files: FileInfo::Multiple(vec![
//                    crate::torrentfile::torrent::File {
//                        length: 500, // File 1 length
//                        paths: vec!["file1.txt".to_string()],
//                    },
//                    crate::torrentfile::torrent::File {
//                        length: 300, // File 2 length
//                        paths: vec!["file2.txt".to_string()],
//                    },
//                    crate::torrentfile::torrent::File {
//                        length: 700, // File 3 length
//                        paths: vec!["file3.txt".to_string()],
//                    },
//                ]),
//                name: "test_torrent".to_string(),
//            },
//            announce: todo!(),
//            announce_list: todo!(),
//            comment: todo!(),
//            creation_date: todo!(),
//            created_by: todo!(),
//            info_hash: todo!(),
//            peer_id: todo!(),
//        }
//    }
//
//    fn mock_torrent_single_file() -> Torrent {
//        Torrent {
//            info: crate::torrentfile::torrent::Info {
//                piece_length: 256,
//                pieces: vec![],
//                files: FileInfo::Single(1000), // Single file length
//                name: "single_file_torrent".to_string(),
//            },
//        }
//    }
//
//    fn setup_test_file(path: &Path) {
//        if path.exists() {
//            remove_file(path).unwrap(); // Clean up if file exists
//        }
//    }
//
//    #[test]
//    fn test_write_single_file() {
//        let torrent = mock_torrent_single_file();
//        let piece = PieceResult {
//            index: 0,
//            buf: vec![1, 2, 3, 4], // Mock piece data
//        };
//        let download_dir = "test_dir".to_string();
//        let path = Path::new(&download_dir).join(&torrent.info.name);
//
//        // Setup: Create directory if it doesn't exist
//        create_dir_all(&download_dir).unwrap();
//
//        setup_test_file(&path); // Clean up if file exists
//
//        // Call write_file
//        let result = write_file(&torrent, piece, download_dir.clone());
//
//        // Check if the file was written correctly
//        assert!(result.is_ok());
//        assert!(path.exists());
//
//        // Open file and verify the content
//        let mut file = OpenOptions::new()
//            .read(true)
//            .write(true)
//            .open(path)
//            .unwrap();
//        let mut buffer = vec![0; 4];
//        file.read_exact(&mut buffer).unwrap();
//
//        assert_eq!(buffer, vec![1, 2, 3, 4]); // Verifying the content
//    }
//
//    #[test]
//    fn test_write_multiple_files() {
//        let torrent = mock_torrent_multiple_files();
//        let piece = PieceResult {
//            index: 1,              // This piece should span across multiple files
//            buf: vec![5, 6, 7, 8], // Mock piece data
//        };
//        let download_dir = "test_dir".to_string();
//
//        // Setup: Create directories for files if they don't exist
//        let file_paths = vec![
//            Path::new(&download_dir).join("file1.txt"),
//            Path::new(&download_dir).join("file2.txt"),
//            Path::new(&download_dir).join("file3.txt"),
//        ];
//
//        for path in &file_paths {
//            create_dir_all(path.parent().unwrap()).unwrap();
//            setup_test_file(path);
//        }
//
//        // Call write_file
//        let result = write_file(&torrent, piece, download_dir.clone());
//
//        // Check if the files were written correctly
//        assert!(result.is_ok());
//
//        // Verify that the data was written to the files
//        for (i, path) in file_paths.iter().enumerate() {
//            let mut file = OpenOptions::new()
//                .read(true)
//                .write(true)
//                .open(path)
//                .unwrap();
//            let mut buffer = vec![0; 4];
//            file.read_exact(&mut buffer).unwrap();
//
//            // Check the content based on the piece index and file offset
//            if i == 0 {
//                assert_eq!(buffer, vec![5, 6, 7, 8]); // Verify that piece data is written to the correct file
//            }
//        }
//    }
//
//    #[test]
//    fn test_write_file_error_handling() {
//        let torrent = mock_torrent_single_file();
//        let piece = PieceResult {
//            index: 0,
//            buf: vec![1, 2, 3, 4], // Mock piece data
//        };
//        let download_dir = "non_existent_dir".to_string(); // A non-existent directory
//
//        // Call write_file and expect an error due to the invalid directory
//        let result = write_file(&torrent, piece, download_dir);
//
//        assert!(result.is_err());
//        assert_eq!(
//            result.err().unwrap(),
//            "No such file or directory (os error 2)"
//        );
//    }
//
//    #[test]
//    fn test_write_multi_file_pieces() {
//        let torrent = mock_torrent_multiple_files();
//        let piece = PieceResult {
//            index: 1,                    // This piece should span across multiple files
//            buf: vec![1, 2, 3, 4, 5, 6], // Mock piece data
//        };
//        let download_dir = "test_dir".to_string();
//
//        // Setup: Create directories for files if they don't exist
//        let file_paths = vec![
//            Path::new(&download_dir).join("file1.txt"),
//            Path::new(&download_dir).join("file2.txt"),
//            Path::new(&download_dir).join("file3.txt"),
//        ];
//
//        for path in &file_paths {
//            create_dir_all(path.parent().unwrap()).unwrap();
//            setup_test_file(path);
//        }
//
//        // Call write_file
//        let result = write_file(&torrent, piece, download_dir.clone());
//
//        // Check if the files were written correctly
//        assert!(result.is_ok());
//
//        // Verify that the data was written to the files
//        for (i, path) in file_paths.iter().enumerate() {
//            let mut file = OpenOptions::new()
//                .read(true)
//                .write(true)
//                .open(path)
//                .unwrap();
//            let mut buffer = vec![0; 6];
//            file.read_exact(&mut buffer).unwrap();
//
//            // Verify the contents of the files
//            if i == 0 {
//                assert_eq!(buffer, vec![1, 2]); // Correct portion of piece data
//            } else if i == 1 {
//                assert_eq!(buffer, vec![3, 4]); // Correct portion of piece data
//            } else {
//                assert_eq!(buffer, vec![5, 6]); // Correct portion of piece data
//            }
//        }
//    }
//}
//
