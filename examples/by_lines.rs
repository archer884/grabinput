extern crate grabinput;

fn main() {
    for line in grabinput::by_lines(std::env::args().nth(1)) {
        print!("{}", line);
    }
}
