# grabinput
Unixy lib for reading from a file or from stdin

[![Build Status](https://travis-ci.org/archer884/grabinput.svg?branch=master)](https://travis-ci.org/archer884/grabinput)

> Note: This library provides no real safety features. If you don't want simple, no-hassle input, don't use it. This is intended to blow up in your face if anything goes wrong. That's a feature, not a bug. Seriously, it doesn't even lock standard input before reading it. Ok? Ok. Just wanted to be sure we were clear on that. :)

## Installation
Add `grabinput = "*"` to your cargo dependencies.

## Usage
This library really is intended to be dirt simple. It doesn't do much--just allows you to skip some typing when you want to read something. Like, say you want to write a program to add up all the integers in a file...

    let sum = grabinput::from_args().with_fallback()
        .filter_map(|n| n.trim().parse().ok())
        .sum::<i32>();

That's your whole program now. I thought about having the library trim newlines from the end of each line, because .NET's similar library functions will do that, but I guess I just figured it was faster to let the user decide--no reason to make them pay for the work if they don't care if it's done or not, right? Anyway...

## License
Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
