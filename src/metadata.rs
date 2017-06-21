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
    pub fn default() -> Metadata {
        lazy_static! {
            static ref DEF_MD: Metadata = Metadata { duration: 4, octave: 3, bpm: 120 };
        }
        *DEF_MD
    }

    pub fn parse(tag_data: &'static str) -> Result<Metadata, ParseError> {
        let mut metadata = Metadata::default();

        for tag in tag_data.split(',') {
            let pair = Metadata::parse_tag(tag)?;

            let key: &str = pair[0];
            let value: i32 = pair[1].parse().expect("Settings values must be integers");

            match key {
                "d" => metadata.duration = value,
                "o" => metadata.octave   = value,
                "b" => metadata.bpm      = value,
                _   => {
                    let msg = format!("{} is not a valid song option setting [dob]", key);
                    return Err(MetadataParseError(msg))
                }
            }
        }

        Ok(metadata)
    }

    fn parse_tag(tag: &'static str) -> Result<Vec<&str>, ParseError> {
        let pair: Vec<&str> = tag.split('=').map(|x| x.trim()).collect();

        match pair.len() {
            2 => Ok(pair),
            _ => {
                let msg = format!("{} is not a valid song metadata tag", tag);
                return Err(MetadataParseError(msg))
            }
        }
    }
}

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "d={},o={},b={}", self.duration, self.octave, self.bpm)
    }
}