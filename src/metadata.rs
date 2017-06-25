use std::fmt;
use super::errors::*;
use super::errors::ParseError::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Metadata {
    pub duration: i32,
    pub octave: i32,
    pub bpm: i32
}

lazy_static! {
    static ref DEFAULT_METADATA: Metadata = Metadata {
        duration: 4,
        octave: 3,
        bpm: 120
    };
}

impl Metadata {
    pub fn default() -> &'static Metadata {
        &DEFAULT_METADATA
    }

    pub fn parse(tag_data: &str) -> ParseResult<Metadata> {
        let mut metadata = DEFAULT_METADATA.clone();

        for tag_str in tag_data.split(',') {
            let (key, value) = Metadata::parse_tag(tag_str)?;

            match key {
                "d" => metadata.duration = value,
                "o" => metadata.octave   = value,
                "b" => metadata.bpm      = value,
                _   => { return Err(MetadataParseError(tag_str)) }
            }
        }

        Ok(metadata)
    }

    fn split_tag(tag: &str) -> ParseResult<Vec<&str>> {
        let pair: Vec<&str> = tag.split('=').map(|x| x.trim()).collect();

        match pair.len() {
            2 => Ok(pair),
            _ => Err(MetadataParseError(tag))
        }
    }

    fn parse_tag(data: &str) -> ParseResult<(&str, i32)> {
        let parts = Metadata::split_tag(data)?;

        let key = parts[0];
        match parts[1].parse::<i32>() {
            Ok(value)  => Ok((key, value)),
            Err(_) => Err(MetadataParseError(data))
        }
    }
}

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "d={},o={},b={}", self.duration, self.octave, self.bpm)
    }
}