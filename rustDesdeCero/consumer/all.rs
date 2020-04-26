fn main() {
    println!("{}",[45, 8, 2, 6]
        .iter()
        .all(|n: &i32| -> bool {*n > 0}) 
    );
}
