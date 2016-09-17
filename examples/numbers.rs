extern crate grabinput;

fn main() {
    let n: i32 = grabinput::from_args().with_fallback()
        .filter_map(|n| n.trim().parse::<i32>().ok())
        .sum();

    println!("{}", n);
}