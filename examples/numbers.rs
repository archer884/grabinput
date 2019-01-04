fn main() {
    let n: i32 = grabinput::default()
        .lines()
        .filter_map(|n| n.parse::<i32>().ok())
        .sum();

    println!("{}", n);
}
