use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let word = &args[1].to_uppercase();
    let letters: Vec<&str> = word.split("").collect();

    println!("{}", letters[1..].join(" "));
    print!("{}", letters[2..].join(" \n"));
}
