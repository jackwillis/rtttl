use std::convert::TryFrom;

use super::errors::*;
use super::errors::ParseError::*;

#[derive(PartialEq, Eq, Debug, Hash)]
#[allow(dead_code)]
pub enum Pitch {
    Rest, C, Cs, D, Ds, E, F, Fs, G, Gs, A, As, B
}

impl<'a> TryFrom<&'a str> for Pitch {
    type Error = ParseError<'a>;

    fn try_from(original: &'a str) -> ParseResult<'a, Pitch> {
        use self::Pitch::*;
        match original.trim().to_uppercase().as_str() {
            "P"  => Ok(Rest),
            "C"  => Ok(C),
            "C#" => Ok(Cs),
            "D"  => Ok(D),
            "D#" => Ok(Ds),
            "E"  => Ok(E),
            "F"  => Ok(F),
            "F#" => Ok(Fs),
            "G"  => Ok(G),
            "G#" => Ok(Gs),
            "A"  => Ok(A),
            "A#" => Ok(As),
            "B"  => Ok(B),
            _ => {
                Err(PitchParseError(original))
            }
        }
    }
}

impl Into<String> for Pitch {
    fn into(self: Pitch) -> String {
        use self::Pitch::*;
        match self {
            Rest => "P",
            C    => "C",
            Cs   => "C#",
            D    => "D",
            Ds   => "D#",
            E    => "E",
            F    => "F",
            Fs   => "F#",
            G    => "G",
            Gs   => "G#",
            A    => "A",
            As   => "A#",
            B    => "B",
        }.to_string()
    }
}