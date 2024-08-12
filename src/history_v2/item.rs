/// Defines a minimum set of functionality that is required to read and write a history item.
pub trait SaveableHistoryItem {
    /// Converts this item to a string suitable for writing to a buffer.
    fn serialize(&self) -> String;
}
