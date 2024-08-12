use std::fs::File;
use std::fs::OpenOptions;
use std::io::Error;
use std::path::Path;

/// A reference to a "history". Usually backed by a single file, and is synchronized between shells.
pub struct HistoryV2 {
    // Used only for appending to. Quick and easy.
    append_only_file: File,
    // Used to read contents of the history.
    rw_file: File,
}

impl HistoryV2 {
    /// Creates a new `HistoryV2`, opening the given `file_path` history file.
    pub fn new(file_path: &str) -> Result<HistoryV2, Error> {
        // Open two versions of the file. One is used exclusively for appending, the other for
        // reading.
        let append_only_file = OpenOptions::new().append(true).open(Path::new(file_path))?;
        let rw_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(Path::new(file_path))?;

        Ok(Self {
            append_only_file,
            rw_file,
        })
    }
}
