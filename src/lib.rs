use std::error::Error;

use crate::config::Config;

pub mod config;
mod executors;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let result = config.query.execute(&config)?;
    match result {
        None => {}
        Some(content) => { println!("{:?}", content) }
    }
    Ok(())
}
