fn main() {
    let arr = [45, 8, -2, 6];

    match arr.iter().max() {
        Some(n) => println!("{}", n),
        _ => (),
    }
}
