use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Stdin};
use std::path::Path;

pub enum InputSource {
    FromFile(BufReader<File>),
    FromStdin(BufReader<Stdin>),
}

impl Iterator for InputSource {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self {
            &mut InputSource::FromFile(ref mut file) => next_line(file),
            &mut InputSource::FromStdin(ref mut stdin) => next_line(stdin),
        }
    }
}

pub fn by_lines<P: AsRef<Path>>(optional_path: Option<P>) -> InputSource {
    match optional_path.and_then(|path| File::open(&path).ok()) {
        None => InputSource::FromStdin(BufReader::new(io::stdin())),
        Some(file) => InputSource::FromFile(BufReader::new(file)),
    }
}

pub fn all<P: AsRef<Path>>(optional_path: Option<P>) -> String {
    match optional_path.and_then(|path| File::open(&path).ok()) {
        None => whole_stream(&mut io::stdin()),
        Some(mut file) => whole_stream(&mut file),
    }
}

#[allow(unused)]
fn next_line<T: BufRead>(reader: &mut T) -> Option<String> {
    let mut buf = String::new();
    match reader.read_line(&mut buf) {
        Ok(0) | Err(_) => None,
        Ok(_) => Some(buf),
    }
}

#[allow(unused)]
fn whole_stream<T: Read>(reader: &mut T) -> String {
    let mut buf = String::new();
    reader.read_to_string(&mut buf);
    buf
}
