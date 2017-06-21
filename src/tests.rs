#![cfg(test)]

use test::Bencher;
use std::convert::TryFrom;
use super::*;

#[bench]
fn bench_parse_song(b: &mut Bencher) {
    b.iter(|| Song::parse("HauntHouse: d=4,o=5,b=108: 2a4, 2e, 2d#, 2b4, 2a4, 2c, 2d, 2a#4"));
}

#[bench]
fn bench_parse_metadata(b: &mut Bencher) {
    b.iter(|| Metadata::parse("d=4,o=5,b=108"));
}

#[bench]
fn bench_parse_melody(b: &mut Bencher) {
    b.iter(|| Melody::parse_default("2a4, 2e, 2d#, 2b4, 2a4, 2c, 2d, 2a#4"));
}

#[bench]
fn bench_parse_note(b: &mut Bencher) {
    b.iter(|| Note::parse("2a4", &Metadata::default()));
}

#[bench]
fn bench_parse_pitch(b: &mut Bencher) {
    b.iter(|| Pitch::try_from("a").unwrap());
}

#[test]
fn test_parse_song_bad() {
    let result = Song::parse("ShortHouse: d=4,o.5,b=108: 2a4, 2e :");

    assert!(result.is_err());
    println!("{:?}", result.err().unwrap());
}

#[test]
fn test_parse_song_good() {
    let result = Song::parse("ShortHouseGood: d=4,o=5,b=108: 2a4, 2e");

    assert!(result.is_ok());

    let song = result.unwrap();

    println!("{}", song);
    println!("{:?}", song);
}