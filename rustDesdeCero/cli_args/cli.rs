fn main() {
    /*let command_line : std::env::Args = std::env::args();

    for argument in command_line {
        println!("[{}]",argument);
    } */
    for a in std::env::args() {
        print!("[{}]=[{}]", a.0, a.1);
    }

    //std::process::exit(107);
}
