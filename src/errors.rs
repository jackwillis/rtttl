#[derive(Debug)]
#[allow(dead_code)]
pub enum ParseError {
    SongParseError(String),
    NoteParseError(String),
    PitchParseError(String),
    MetadataParseError(String)
}