use std::env;
use std::process;

use minigrep::Config;

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

    if let Err(e) = minigrep::run(config) {
        println!("Application error {}", e);
        process::exit(1);
    };
}
