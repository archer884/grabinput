use read;
use std::fs::File;
use std::io::{self, BufReader, Stdin};
use std::path::Path;

/// `FromFile` provides a convenient wrapper for an optional file.
///
/// `FromFile` provides an iterator over lines found in the file or, 
/// alternatively, a method providing access to the whole stream as a single 
/// string.
///
/// Additionally, it is possible to convert to an object providing access to 
/// an optional file or to standard in as a backup. 
#[derive(Default)]
pub struct FromFile {
    file: Option<BufReader<File>>,
}

impl FromFile {
    /// Creates a new, empty, FromFile struct.
    pub fn new() -> FromFile {
        FromFile { file: None }
    }

    /// Creates a new FromFile struct based on the provided path.
    ///
    /// If the provided path cannot be opened for reading, the FromFile struct 
    /// will be created without any file handle and will return an empty string 
    /// read or iterated.
    pub fn from_path<T: AsRef<Path>>(path: T) -> FromFile {
        FromFile { file: File::open(path).ok().map(|f| BufReader::new(f)) }
    }

    /// Creates a new WithFallback struct based on the FromFile struct.
    pub fn with_fallback(self) -> WithFallback {
        WithFallback {
            file: self.file,
            stdin: None,
        }
    }

    /// Reads the entire contents of the file stream into a string.
    pub fn all(&mut self) -> String {
        match self.file {
            None => String::new(),
            Some(ref mut read) => read::whole_stream(read),
        }
    }
}

/// `WithFallBack` wraps an optional file stream and standard input stream.
///
/// WithFallback provides an iterator over lines found in the input streams
/// or, alternatively, a method providing access to the whole stream as a 
/// single string.
///
/// Fallback behavior is such that standard input will not be opened unless 
/// the provided file cannot be opened. Additionally, only one of the two 
/// streams will ever be used.
pub struct WithFallback {
    file: Option<BufReader<File>>,
    stdin: Option<BufReader<Stdin>>,
}

impl WithFallback {
    /// Reads the entire content of the appropriate stream into a string.
    pub fn all(&mut self) -> String {
        if let Some(ref mut file) = self.file {
            return read::whole_stream(file);
        }

        if let Some(ref mut stdin) = self.stdin {
            return read::whole_stream(stdin);
        }

        String::new()
    }
}

impl Iterator for FromFile {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.file {
            None => None,
            Some(ref mut file) => read::next_line(file),
        }
    }
}

impl Iterator for WithFallback {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut file) = self.file {
            return read::next_line(file);
        }

        if let Some(ref mut stdin) = self.stdin {
            return read::next_line(stdin);
        }

        let mut stdin = BufReader::new(io::stdin());
        let ret = read::next_line(&mut stdin);
        self.stdin = Some(stdin);
        ret
    }
}
