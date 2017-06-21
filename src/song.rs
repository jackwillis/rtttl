use std::fmt;
use super::*;
use super::errors::*;
use super::errors::ParseError::*;

#[derive(PartialEq, Eq, Debug)]
pub struct Song {
    pub name: &'static str,
    pub metadata: Metadata,
    pub melody: Melody
}

impl fmt::Display for Song {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}: {}", self.name, self.metadata, self.melody)
    }
}

impl Song {
    pub fn parse(data: &'static str) -> Result<Song, ParseError> {
        let parts = Song::split_into_parts(&data)?;

        let name = parts[0];
        let metadata = Metadata::parse(parts[1])?;
        let melody = Melody::parse(parts[2], &metadata)?;

        Ok(Song { name: name, metadata: metadata, melody: melody })
    }

    fn split_into_parts(data: &'static str) -> Result<Vec<&str>, ParseError> {
        let parts: Vec<&str> = data.split(':').collect();

        match parts.len() {
            3 => Ok(parts),
            _ => {
                let msg = format!("Malformed song data {:?}: too many colons.", data);
                Err(SongParseError(msg))
            }
        }
    }
}