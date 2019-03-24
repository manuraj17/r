use std::error::Error;

pub struct Config {
    pub word: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Usage: r <word>");
        }

        let word = args[1].clone();

        Ok(Config { word })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let word = config.word.to_uppercase();

    let letters: Vec<&str> = word.split("").collect();

    println!("{}", letters[1..].join(" "));
    print!("{}", letters[2..].join(" \n"));
    Ok(())
}

