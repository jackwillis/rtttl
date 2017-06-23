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
    pub dotted: bool
}

impl Note {
    fn split_note_data(data: &str) -> Result<regex::Captures, ParseError> {
        lazy_static! {
            static ref NOTE_RE: Regex = Regex::new(r"(-?\d+)?([a-gp]#?)(-?\d+)?(\.)?").unwrap();
        }

        match (*NOTE_RE).captures(data) {
            Some(cap) => Ok(cap),
            None => Err(NoteParseError(data.to_string()))
        }
    }

    pub fn parse(data: &str, options: &Metadata) -> Result<Note, ParseError> {
        let parts = Note::split_note_data(&data)?;

        let octave = match parts.get(1) {
            Some(n) =>
                match n.as_str().parse::<i32>() {
                    Ok(parsed) => parsed,
                    _ => options.octave
                },
            None => options.octave
        };

        let pitch = Pitch::try_from(&parts[2])?;

        let duration = match parts.get(3) {
            Some(n) =>
                match n.as_str().parse::<i32>() {
                    Ok(parsed) => parsed,
                    _ => options.duration
                },
            None => options.duration
        };

        let dotted = parts.get(4).is_some();

        Ok(Note { pitch: pitch, octave: octave, duration: duration, dotted: dotted })
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = write!(f, "{}{:?}{:?}", self.octave, self.pitch, self.duration);

        if self.dotted {
            write!(f, ".")
        }
        else {
            res
        }
    }
}