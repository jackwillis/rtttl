use std::fmt;
use super::errors::*;
use super::errors::ParseError::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Metadata {
    pub duration: i32,
    pub octave: i32,
    pub bpm: i32
}

impl Metadata {
    pub fn default() -> &'static Metadata {
        lazy_static! {
            static ref DEF_MD: Metadata = Metadata {
                duration: 4,
                octave: 3,
                bpm: 120
            };
        }
        &DEF_MD
    }

    pub fn parse(tag_data: &str) -> Result<Metadata, ParseError> {
        let mut metadata = Metadata::default().clone();

        for tag_str in tag_data.split(',') {
            let (key, value) = Metadata::parse_tag(tag_str)?;

            match key {
                "d" => metadata.duration = value,
                "o" => metadata.octave   = value,
                "b" => metadata.bpm      = value,
                _   => { return Err(MetadataParseError(tag_str.to_string())) }
            }
        }

        Ok(metadata)
    }

    fn split_tag(tag: &str) -> Result<Vec<&str>, ParseError> {
        let pair: Vec<&str> = tag.split('=').map(|x| x.trim()).collect();

        match pair.len() {
            2 => Ok(pair),
            _ => Err(MetadataParseError(tag.to_string()))
        }
    }

    fn parse_tag(data: &str) -> Result<(&str, i32), ParseError> {
        let parts = Metadata::split_tag(data)?;

        match parts[1].parse::<i32>() {
            Ok(n)  => Ok((parts[0], n)),
            Err(_) => Err(MetadataParseError(data.to_string()))
        }
    }
}

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "d={},o={},b={}", self.duration, self.octave, self.bpm)
    }
}