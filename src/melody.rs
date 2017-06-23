use std::fmt;
use super::*;

#[derive(PartialEq, Eq, Debug)]
pub struct Melody(pub Vec<Note>);

impl Melody {
    pub fn parse_default(data: &str) -> Result<Melody, ParseError> {
        Melody::parse(data, &Metadata::default())
    }

    pub fn parse(data: &str, metadata: &Metadata) -> Result<Melody, ParseError> {
        let mut notes: Vec<Note> = vec![];

        let note_strs = data.split(",").map(|d| d.trim());

        for str in note_strs {
            let note = Note::parse(str, &metadata)?;
            notes.push(note);
        }

        Ok(Melody(notes))
    }
}

impl fmt::Display for Melody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fmt_notes: Vec<String> = self.0.iter().map(|note| format!("{}", note)).collect();
        write!(f, "{}", fmt_notes.join(", "))
    }
}