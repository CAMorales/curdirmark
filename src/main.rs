use std::{env, process};

use curdirmark::config::Config;


fn main() {
    let config = Config::build(env::args(), env::current_dir().expect("Failed to get current working directory"))
        .unwrap_or_else(|_| {
        process::exit(1);
    });
    if let Err(e) = curdirmark::run(config) {
        eprintln!("Application error!: {e}");
        process::exit(1);
    }

}
