#[derive(Debug)]
#[allow(dead_code)]
pub enum ParseError {
    SongParseError(String), //format!("Malformed song data {:?}: too many colons.", data)
    NoteParseError(String),
    PitchParseError(String),
    MetadataParseError(String) // format!("{} is not a valid song option tag [dob]=i32", self.0)
}