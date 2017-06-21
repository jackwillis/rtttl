#![feature(test)]
#![feature(try_from)]

extern crate regex;
extern crate test;

#[macro_use]
extern crate lazy_static;

mod song;
pub use self::song::Song;

mod metadata;
pub use self::metadata::Metadata;

mod melody;
pub use self::melody::Melody;

mod note;
pub use self::note::Note;

mod pitch;
pub use self::pitch::Pitch;

mod errors;
pub use self::errors::ParseError;

mod tests;