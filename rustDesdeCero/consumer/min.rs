
fn main(){
    let arr = [45, 8, -2, 6];

    match arr.iter().min(){
        Some(n) => println!("{}", n),
        _ => (),
    }
}