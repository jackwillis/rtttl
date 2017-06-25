use std::fmt;
use super::*;
use super::errors::*;
use super::errors::ParseError::*;

#[derive(PartialEq, Eq, Debug)]
pub struct Song<'a> {
    pub name: &'a str,
    pub metadata: Metadata,
    pub melody: Melody
}

impl<'a> fmt::Display for Song<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}: {}", self.name, self.metadata, self.melody)
    }
}

impl<'a> Song<'a> {
    pub fn parse(data: &'a str) -> ParseResult<Song> {
        let parts = Song::split_into_parts(&data)?;

        let name = parts[0];
        let metadata = Metadata::parse(parts[1])?;
        let melody = Melody::parse(parts[2], &metadata)?;

        Ok(Song { name: name, metadata: metadata, melody: melody })
    }

    fn split_into_parts(data: &'a str) -> ParseResult<Vec<&'a str>> {
        let parts: Vec<&'a str> = data.split(':').collect();

        match parts.len() {
            3 => Ok(parts),
            _ => Err(SongParseError(data))
        }
    }
}