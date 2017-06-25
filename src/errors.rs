#[derive(Debug)]
#[allow(dead_code)]
pub enum ParseError<'a> {
    SongParseError(&'a str), //format!("Malformed song data {:?}: too many colons.", data)
    NoteParseError(&'a str),
    PitchParseError(&'a str),
    MetadataParseError(&'a str) // format!("{} is not a valid song option tag [dob]=i32", self.0)
}

pub type ParseResult<'a, T> = Result<T, ParseError<'a>>;