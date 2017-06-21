use std::fmt;
use std::convert::TryFrom;
use regex::Regex;
use super::*;
use super::errors::*;
use super::errors::ParseError::*;

#[derive(PartialEq, Eq, Debug)]
pub struct Note {
    pub pitch: Pitch,
    pub octave: i32,
    pub duration: i32,
}

fn split_note_data(data: &str) -> Result<regex::Captures, ParseError> {
    lazy_static! {
        static ref NOTE_RE: Regex = Regex::new(r"(-?\d+)([a-gp]#?)(-?\d+)?").unwrap();
    }

    match (*NOTE_RE).captures(data) {
        Some(cap) => Ok(cap),
        None => Err(NoteParseError(data.to_string()))
    }
}

impl Note {
    pub fn parse(data: &str, options: &Metadata) -> Result<Note, ParseError> {
        let parts = split_note_data(&data)?;
        let pitch = Pitch::try_from(&parts[2])?;
        let octave: i32 = parts[1].parse().unwrap();

        let duration: i32 = if parts.get(3).is_none() {
            options.duration
        }
        else {
            parts[3].parse().unwrap_or(options.duration)
        };

        Ok(Note { pitch: pitch, octave: octave, duration: duration })
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{:?}{}", self.octave, self.pitch, self.duration)
    }
}