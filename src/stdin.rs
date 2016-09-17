use read;
use std::io::{self, BufReader, Stdin};

/// `FromStdin` wraps a buffered `Stdin`.
///
/// `FromStdin` provides an iterator over lines found in the input stream or,
/// alternatively, a method providing access to the whole stream as a single
/// string.
pub struct FromStdin(BufReader<Stdin>);

impl FromStdin {
    /// Creates a new instance of FromStdin.
    pub fn new() -> FromStdin {
        FromStdin(BufReader::new(io::stdin()))
    }

    /// Reads the entire contents of the standard input stream into a string.
    pub fn all(&mut self) -> String {
        read::whole_stream(&mut self.0)
    }
}

impl Default for FromStdin {
    fn default() -> FromStdin {
        FromStdin::new()
    }
}

impl Iterator for FromStdin {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        read::next_line(&mut self.0)
    } 
}
