use std::io::Error;

/// A reference to a "history". Usually backed by a single file, and is synchronized between shells.
pub struct HistoryV2 {
    file_path: String,
}

impl HistoryV2 {
    /// Creates a new `HistoryV2`, opening the given `file_path` history file.
    pub fn new(file_path: &str) -> Result<HistoryV2, Error> {
        Ok(Self {
            file_path: file_path.to_owned(),
        })
    }
}
