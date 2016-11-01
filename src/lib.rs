#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

//! This library really is intended to be dirt simple. It doesn't do much--just
//! allows you to skip some typing when you want to read something. Like, say
//! you want to write a program to add up all the integers in a file...
//!
//! ```rust
//! extern crate grabinput;
//!
//! let sum: i32 = grabinput::from_args().with_fallback()
//!     .filter_map(|n| n.trim().parse::<i32>().ok())
//!     .sum();
//! ```
//!
//! That's your whole program now. I thought about having the library trim
//! newlines from the end of each line, because .NET's similar library functions
//! will do that, but I guess I just figured it was faster to let the user
//! decide--no reason to make them pay for the work if they don't care if it's
//! done or not, right? Anyway...

mod file;
mod read;
mod stdin;

pub use file::*;
use std::path::Path;
pub use stdin::FromStdin;

/// Creates an input handle based on `std::env::args().nth(1)`.
///
/// The assumption here is that your program is executed as `<program> <file>`,
/// in which case the 1st (not 0th) argument names the file to be read. See
/// [`from_path`] for support for custom paths.
///
/// [`from_path`]: fn.from_path.html
pub fn from_args() -> FromFile {
    std::env::args()
        .nth(1)
        .map(FromFile::from_path)
        .unwrap_or_else(FromFile::new)
}

/// Creates an input handle based on the provided path.
///
/// To create an input handle based on an optional path, see [`from_optional_path`].
///
/// [`from_optional_path`]: fn.from_optional_path.html
pub fn from_path<T: AsRef<Path>>(path: T) -> FromFile {
    FromFile::from_path(path)
}

/// Creates an input handle based on an optional path.
pub fn from_optional_path<T: AsRef<Path>>(path: Option<T>) -> FromFile {
    match path {
        None => FromFile::default(),
        Some(ref path) => FromFile::from_path(path),
    }
}

/// Creates an input handle based on standard in.
pub fn from_stdin() -> FromStdin {
    FromStdin::default()
}
