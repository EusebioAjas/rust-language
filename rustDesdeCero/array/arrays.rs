fn main() {
    let mut words = ["This", "is", "a", "sentence"];
    words[2] = "a nice";

    println!("{:?}", words);

    let result = f(-4.);
    println!("{}",result);
}

fn f(x: f64) -> f64 {
    if x <= 0. {
        return 0.;
    }
    return x + 3.;
}
