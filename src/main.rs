use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprint!("ERROR: {err}");
        process::exit(1)
    });
    println!("{}, {}", config.query, config.filename);
    match minigrep::run(config) {
        Err(err) => {
            eprint!("{err}");
            process::exit(1)
        }
        _ => {}
    }
}
