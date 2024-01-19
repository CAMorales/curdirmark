use std::error::Error;

use crate::config::Config;

pub mod config;
mod executors;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    config.query.execute(&config)?;
    Ok(())
}
