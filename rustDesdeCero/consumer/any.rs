fn main() {
    let s = "Hello, World";
    let ch = 'R';

    println!("\"{}\" {} '{}'", s,
        if s.chars().any(|c| c == ch){
            "contains"
        }else {
            "does not contain"
        }, 
        ch
    );
}
