fn main() {
    let v = [66, -8, 43, 19, 0, -31]
        .iter()
        .filter(|x| {print!("Fil: {} ",x); **x > 2})
        .map(|x|{print!("Map: {} ",x);*x *2})
        .collect::<Vec<_>>();
    println!("{:?}",v);

}
