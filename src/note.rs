use std::fmt;
use std::convert::TryFrom;
use std::str::FromStr;
use regex::Regex;
use num_rational::Ratio;
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

fn parse_match_or_default<T>(comp: Option<regex::Match>, default: T) -> T where T: FromStr {
    match comp {
        Some(n) =>
            match n.as_str().parse::<T>() {
                Ok(parsed) => parsed,
                _ => default
            },
        None => default
    }
}

lazy_static! {
    static ref NOTE_REGEX: Regex = Regex::new(r"(-?\d+)?([a-gp]#?)(-?\d+)?(\.)?").unwrap();
}

impl Note {
    fn split_note_data(data: &str) -> ParseResult<regex::Captures> {
        match NOTE_REGEX.captures(data) {
            Some(cap) => Ok(cap),
            None => Err(NoteParseError(data))
        }
    }

    pub fn parse<'a>(data: &'a str, options: &Metadata) -> ParseResult<'a, Note> {
        let parts = Note::split_note_data(&data)?;

        let octave = parse_match_or_default::<i32>(parts.get(1), options.octave);
        let pitch = Pitch::try_from(parts.get(2).unwrap().as_str())?;

        let duration = parse_match_or_default::<i32>(parts.get(3), options.duration);
        let dotted = parts.get(4).is_some();

        Ok(Note { pitch: pitch, octave: octave, duration: duration, dotted: dotted })
    }

    pub fn parse_default(data: &str) -> ParseResult<Note> {
        Note::parse(&data, Metadata::default())
    }

    pub fn fraction(&self) -> Ratio<i32> {
        if self.dotted {
            Ratio::new(3, self.duration * 2)
        }
        else {
            Ratio::new(1, self.duration)
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.dotted {
            write!(f, "{}{:?}{:?}.", self.octave, self.pitch, self.duration)
        }
        else {
            write!(f, "{}{:?}{:?}", self.octave, self.pitch, self.duration)
        }
    }
}