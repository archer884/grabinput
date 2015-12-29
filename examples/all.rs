extern crate grabinput;

fn main() {
    println!("{}", grabinput::all(std::env::args().nth(1)));
}
