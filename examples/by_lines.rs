extern crate grabinput;

fn main() {
    for line in grabinput::from_args().with_fallback() {
        print!("{}", line);
    }
}
