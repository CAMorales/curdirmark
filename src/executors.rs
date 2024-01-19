use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter};
use std::io::Write;
use std::path::Path;

use crate::config::{Config, print_usage};

pub trait BookMarkAction: Debug {
    fn execute(&self, config: &Config) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug)]
pub struct Save {}

impl BookMarkAction for Save {
    fn execute(&self, config: &Config) -> Result<(), Box<dyn Error>> {
        let mut bookmarks = load_bookmarks(&config)?;
        bookmarks.insert((&config.bookmark as &str).to_string(), config.path.to_string_lossy().to_string());
        // let mut file= file.try_clone()?;
        write_bookmarks(&config, &bookmarks)?;

        Ok(())
    }
}

fn load_bookmarks(config: &Config) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        // .open(Path::new(&*Rc::clone(&config.local_db)))?;
        .open(Path::new(&*config.local_db))?;
    let reader = BufReader::new(file);
    let mut contents = HashMap::new();

    // If the file exists, load its contents into the HashMap
    for line in reader.lines() {
        let line = line?;
        // Split the line into key-value pairs based on your delimiter
        let parts: Vec<&str> = line.split('=').collect(); // Example using "=" as delimiter
        if parts.len() == 2 {
            let key = parts[0].into();
            let value = parts[1].into();
            contents.insert(key, value);
        }

    }

    Ok(contents)
}

fn write_bookmarks(config: &Config, contents: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        // .open(Path::new(&*Rc::clone(&config.local_db)))?;
        .open(Path::new(&*config.local_db))?;
    let mut writer = BufWriter::new(file);
    for (key, value) in contents {
        writeln!(&mut writer, "{}={}", key, value)?;
    }
    Ok(())
}

#[derive(Debug)]
pub struct Help {}

impl BookMarkAction for Help {
    fn execute(&self, config: &Config) -> Result<(), Box<dyn Error>> {
        print_usage(&config.program, &config.options);
        Ok(())
    }
}

#[derive(Debug)]
pub struct List {}

impl BookMarkAction for List {
    fn execute(&self, config: &Config) -> Result<(), Box<dyn Error>> {
        let bookmarks = load_bookmarks(&config)?;
        println!("{:?}", bookmarks);
        Ok(())
    }
}

#[derive(Debug)]
pub struct RemoveDB {}

impl BookMarkAction for RemoveDB {
    fn execute(&self, _config: &Config) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

#[derive(Debug)]
pub struct ShowBookmark {}

impl BookMarkAction for ShowBookmark {
    fn execute(&self, config: &Config) -> Result<(), Box<dyn Error>> {
        let bookmarks = load_bookmarks(&config)?;
        if let Some(path) = bookmarks.get(&config.bookmark as &str) {
            println!("{}", path);
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct DeleteBookmark {}

impl BookMarkAction for DeleteBookmark {
    fn execute(&self, config: &Config) -> Result<(), Box<dyn Error>> {
        let mut bookmarks = load_bookmarks(&config)?;
        bookmarks.remove(&config.bookmark as &str);
        // let mut file= file.try_clone()?;
        write_bookmarks(&config, &bookmarks)?;

        Ok(())
    }
}

