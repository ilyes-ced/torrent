// mapping to which files each piece belongs to

use crate::torrentfile::torrent::FileInfo::{Multiple, Single};
use crate::Torrent;
use std::cmp::{max, min};

#[derive(Debug)]
pub struct Mapping {
    pub file_index: usize,
    pub file_write_offset: u64,
    pub piece_write_len: u64,
}

pub fn mapping(torrent: &Torrent, piece_index: u32) -> Result<Vec<Mapping>, String> {
    let files = match &torrent.info.files {
        Multiple(files) => files,
        Single(_) => return Err(String::from("we cant accept single files here")),
        // should never happen
        // this was for figuring out the size of the last piece because it was needed for the final piece which is smaller than default piece size for reading already downloaded pieces

        // ugly solution to a small problem yes
        //Single(len) => &[Files {
        //    paths: [torrent.info.name.clone()].to_vec(),
        //    length: *len,
        //}]
        //.to_vec(),
    };

    let mut piece_to_file_mapping = Vec::new();
    let p_len = torrent.info.piece_length;
    let p_ind = piece_index as u64;

    let mut files_len: Vec<u64> = Vec::new();
    for file in files {
        files_len.push(file.length);
    }

    let piece_start = p_len * p_ind;
    let piece_end = p_len * p_ind + p_len;

    let mut cumulative_file_length: u64 = 0;
    for (i, file) in files.iter().enumerate() {
        //file bounds

        let file_start = cumulative_file_length;
        let file_end = cumulative_file_length + file.length;

        if piece_start < file_end && piece_end > file_start {
            let file_offset = max(piece_start, file_start);
            let length = min(piece_end, file_end) - file_offset;

            piece_to_file_mapping.push(Mapping {
                file_index: i,
                file_write_offset: file_offset - file_start,
                piece_write_len: length,
            })
        }

        cumulative_file_length += file.length;
    }

    Ok(piece_to_file_mapping)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::torrentfile::torrent::{FileInfo, Files, Torrent, TorrentInfo};

    // Helper function to create a Torrent with multiple files
    fn mock_torrent_multiple_files() -> Torrent {
        Torrent {
            info: TorrentInfo {
                piece_length: 256, // Example piece length
                pieces: vec![],    // Mock, as we don't need actual pieces for the test
                files: FileInfo::Multiple(vec![
                    Files {
                        length: 500, // File 1 length
                        paths: vec!["file1.txt".to_string()],
                    },
                    Files {
                        length: 300, // File 2 length
                        paths: vec!["file2.txt".to_string()],
                    },
                    Files {
                        length: 700, // File 3 length
                        paths: vec!["file3.txt".to_string()],
                    },
                ]),
                name: "test_torrent".to_string(),
            },
            announce: todo!(),
            announce_list: todo!(),
            comment: todo!(),
            creation_date: todo!(),
            created_by: todo!(),
            info_hash: todo!(),
            peer_id: todo!(),
        }
    }

    // Helper function to create a Torrent with a single file
    fn mock_torrent_single_file() -> Torrent {
        Torrent {
            info: TorrentInfo {
                piece_length: 256,
                pieces: vec![],
                files: FileInfo::Single(1000), // Single file length
                name: "single_file_torrent".to_string(),
            },
            announce: todo!(),
            announce_list: todo!(),
            comment: todo!(),
            creation_date: todo!(),
            created_by: todo!(),
            info_hash: todo!(),
            peer_id: todo!(),
        }
    }

    #[test]
    fn test_mapping_multiple_files_first_piece() {
        let torrent = mock_torrent_multiple_files();

        // The first piece starts at index 0
        let result = mapping(&torrent, 0);

        assert!(result.is_ok());

        let mappings = result.unwrap();
        assert_eq!(mappings.len(), 2); // We expect 2 mappings for the first piece

        // Check first mapping (File 1)
        assert_eq!(mappings[0].file_index, 0);
        assert_eq!(mappings[0].file_write_offset, 0); // Starts at the beginning of file 1
        assert_eq!(mappings[0].piece_write_len, 256); // First piece length

        // Check second mapping (File 2)
        assert_eq!(mappings[1].file_index, 1);
        assert_eq!(mappings[1].file_write_offset, 0); // Starts at the beginning of file 2
        assert_eq!(mappings[1].piece_write_len, 44); // The remaining piece length for file 2 (256 - (500 - 256))
    }

    #[test]
    fn test_mapping_multiple_files_middle_piece() {
        let torrent = mock_torrent_multiple_files();

        // The middle piece (piece_index 1) starts at index 1
        let result = mapping(&torrent, 1);

        assert!(result.is_ok());

        let mappings = result.unwrap();
        assert_eq!(mappings.len(), 2); // We expect 2 mappings for this piece

        // Check first mapping (File 2)
        assert_eq!(mappings[0].file_index, 1);
        assert_eq!(mappings[0].file_write_offset, 0); // Starts at the beginning of file 2
        assert_eq!(mappings[0].piece_write_len, 256); // The piece length of 256

        // Check second mapping (File 3)
        assert_eq!(mappings[1].file_index, 2);
        assert_eq!(mappings[1].file_write_offset, 0); // Starts at the beginning of file 3
        assert_eq!(mappings[1].piece_write_len, 44); // The remaining piece length for file 3
    }

    #[test]
    fn test_mapping_multiple_files_last_piece() {
        let torrent = mock_torrent_multiple_files();

        // The last piece will involve file 3, as it exceeds the combined length of the first two files
        let piece_index = 3; // Last piece
        let result = mapping(&torrent, piece_index);

        assert!(result.is_ok());

        let mappings = result.unwrap();
        assert_eq!(mappings.len(), 1); // Only 1 mapping for the last piece

        // Check first mapping (File 3)
        assert_eq!(mappings[0].file_index, 2);
        assert_eq!(mappings[0].file_write_offset, 0); // Starts at the beginning of file 3
        assert_eq!(mappings[0].piece_write_len, 256); // The remaining piece length for file 3
    }

    #[test]
    fn test_mapping_single_file_error() {
        let torrent = mock_torrent_single_file();

        // The mapping function should return an error for single file torrents
        let result = mapping(&torrent, 0);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "we cant accept single files here");
    }

    #[test]
    fn test_mapping_empty_files_error() {
        let torrent = Torrent {
            info: TorrentInfo {
                piece_length: 256,
                pieces: vec![],
                files: FileInfo::Multiple(vec![]), // Empty files list
                name: "empty_files_torrent".to_string(),
            },
            announce: todo!(),
            announce_list: todo!(),
            comment: todo!(),
            creation_date: todo!(),
            created_by: todo!(),
            info_hash: todo!(),
            peer_id: todo!(),
        };

        // The mapping function should return an empty mapping when there are no files
        let result = mapping(&torrent, 0);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_mapping_out_of_bounds_piece() {
        let torrent = mock_torrent_multiple_files();

        // Piece index that is out of bounds (too high)
        let result = mapping(&torrent, 10);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Piece index is out of bounds");
    }
}
