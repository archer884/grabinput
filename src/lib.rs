use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read};
use std::iter;
use std::path::{Path, PathBuf};

type Lines = Box<Iterator<Item = String>>;

/// Constructs a grabber that will check the given command line argument for 
/// a path to read from.
pub fn args(n: usize) -> Grabber {
    Grabber::args(n)
}

/// Constructs a grabber that will first check the first command line argument
/// for a path to read from and then fall back to stdin.
pub fn default() -> Grabber {
    Grabber::default()
}

/// Constructs a grabber that will attempt to read from the provided path.
pub fn file(path: impl Into<PathBuf>) -> Grabber {
    Grabber::file(path)
}

/// Constructs a grabber that will read from stdin.
pub fn stdin() -> Grabber {
    Grabber::stdin()
}

/// Represents the primary and optional fallback strategy for reading input.
pub struct Grabber {
    mode: GrabberMode,
    fallback_mode: Option<GrabberMode>,
}

enum GrabberMode {
    Args(usize),
    File(PathBuf),
    Stdin,
}

impl Grabber {
    fn new() -> Grabber {
        Grabber {
            mode: GrabberMode::Args(1),
            fallback_mode: Some(GrabberMode::Stdin),
        }
    }

    fn args(n: usize) -> Grabber {
        Grabber {
            mode: GrabberMode::Args(n),
            fallback_mode: None,
        }
    }

    fn file(path: impl Into<PathBuf>) -> Grabber {
        Grabber {
            mode: GrabberMode::File(path.into()),
            fallback_mode: None,
        }
    }

    fn stdin() -> Grabber {
        Grabber {
            mode: GrabberMode::Stdin,
            fallback_mode: None,
        }
    }

    /// Adds stdin as a fallback method.
    pub fn or_stdin(mut self) -> Grabber {
        match self.mode {
            // Cannot apply fallback of stdin to stdin.
            GrabberMode::Stdin => self,

            _ => {
                self.fallback_mode = Some(GrabberMode::Stdin);
                self
            }
        }
    }

    pub fn all(self) -> String {
        fn args_all(n: usize) -> Option<String> {
            let path = env::args().nth(n)?;
            fs::read_to_string(path).ok()
        }

        fn file_all(path: impl AsRef<Path>) -> Option<String> {
            fs::read_to_string(path).ok()
        }

        fn stdin_all() -> String {
            let mut buf = String::new();
            let mut handle = io::stdin();
            let _ = handle.read_to_string(&mut buf);

            buf
        }

        match self.mode {
            GrabberMode::Args(n) => match args_all(n) {
                Some(buf) => buf,
                None => match self.fallback_mode {
                    Some(GrabberMode::File(path)) => file_all(path).expect("Unable to read input"),
                    Some(GrabberMode::Stdin) => stdin_all(),

                    _ => String::new(),
                },
            },

            GrabberMode::File(path) => match file_all(path) {
                Some(buf) => buf,
                None => match self.fallback_mode {
                    Some(GrabberMode::Args(n)) => args_all(n).expect("Unable to read input"),
                    Some(GrabberMode::Stdin) => stdin_all(),

                    _ => String::new(),
                },
            },

            GrabberMode::Stdin => stdin_all(),
        }
    }

    pub fn lines(self) -> Lines {
        fn args_lines(n: usize) -> Option<Lines> {
            let path = env::args().nth(n)?;
            let file = File::open(path).ok()?;

            Some(Box::new(
                BufReader::new(file).lines().filter_map(Result::ok),
            ))
        }

        fn file_lines(path: impl AsRef<Path>) -> Option<Lines> {
            let file = File::open(path).ok()?;

            Some(Box::new(
                BufReader::new(file).lines().filter_map(Result::ok),
            ))
        }

        fn stdin_lines() -> Lines {
            let reader = BufReader::new(io::stdin());
            Box::new(reader.lines().filter_map(Result::ok))
        }

        match self.mode {
            GrabberMode::Args(n) => match args_lines(n) {
                Some(lines) => lines,
                None => match self.fallback_mode {
                    Some(GrabberMode::Stdin) => stdin_lines(),
                    Some(GrabberMode::File(path)) => {
                        file_lines(path).expect("Unable to read input")
                    }

                    _ => Box::new(iter::empty()),
                },
            },

            GrabberMode::File(path) => match file_lines(path) {
                Some(lines) => lines,
                None => match self.fallback_mode {
                    Some(GrabberMode::Stdin) => stdin_lines(),
                    Some(GrabberMode::Args(n)) => args_lines(n).expect("Unable to read input"),

                    _ => Box::new(iter::empty()),
                },
            },

            GrabberMode::Stdin => stdin_lines(),
        }
    }
}

impl Default for Grabber {
    fn default() -> Self {
        Self::new()
    }
}
