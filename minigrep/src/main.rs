use std::env;
use std::fs;
use std::process;

fn main() {
    // 実行時引数の取得
    let args: Vec<String> = env::args().collect();

    // let (query, filename) = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments {}", err);
        process::exit(1);
    });

    println!("searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    print!("The contents is:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
