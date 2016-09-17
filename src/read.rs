use std::io::{BufRead, Read};

#[allow(unused)]
pub fn next_line<T: BufRead>(reader: &mut T) -> Option<String> {
    let mut buf = String::new();
    match reader.read_line(&mut buf) {
        Ok(0) | Err(_) => None,
        Ok(_) => Some(buf),
    }
}

#[allow(unused)]
pub fn whole_stream<T: Read>(reader: &mut T) -> String {
    let mut buf = String::new();
    reader.read_to_string(&mut buf);
    buf
}
