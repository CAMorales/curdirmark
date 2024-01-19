use std::env;
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use std::rc::Rc;

use getopts;
use getopts::Options;

use crate::executors::{BookMarkAction, DeleteBookmark, Help, List, RemoveDB, Save, ShowBookmark};

pub struct Config {
    pub local_db: String,
    pub program: Rc<str>,
    pub query: Box<dyn BookMarkAction>,
    pub bookmark: Rc<str>,
    pub path: PathBuf,
    pub options: Options,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item=String>, cur_dir: PathBuf) -> Result<Config, String> {
        let program = args.next().unwrap_or_default().into();
        let options = gen_options();
        let path = cur_dir;

        let matches = options.parse(args).map_err(|f| {
                eprintln!("{}", f);
                print_usage(&program, &options);
                f.to_string()
        })?;

        let home = env::var("HOME").unwrap_or(String::from("."));

        let local_db = matches.opt_str("b")
            .unwrap_or(home + "/.curdirmarkdb");

        let mut query: Box<dyn BookMarkAction> = Box::new(Help{});

        let mut bookmark = "".into();

        if matches.opt_present("h") {
            return Ok(Config {
                program,
                local_db,
                query,
                bookmark,
                path,
                options,
            });
        }

        if matches.opt_present("l") {
            query = Box::new(List{});
        } else if matches.opt_present("r") {
            query = Box::new(RemoveDB{});
        } else {
            let result = determine_query_and_bookmark(&matches);
            query = result.0;
            bookmark = result.1;
        }



        Ok(Config {
            program,
            local_db,
            query,
            bookmark,
            path,
            options,
        })
    }
}

impl Debug for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("local_db", &self.local_db)
            .field("program", &self.program)
            .field("query", &self.query)
            .field("bookmark", &self.bookmark)
            .field("path", &self.path)
            .finish()
    }
}
fn determine_query_and_bookmark(matches: &getopts::Matches) -> (Box<dyn BookMarkAction>, Rc<str>) {
    match (matches.opt_str("s"), matches.opt_str("o"), matches.opt_str("d")) {
        (Some(value), _, _) => (Box::new(Save{}), value.into()),
        (_, Some(value), _) => (Box::new(ShowBookmark{}), value.into()),
        (_, _, Some(value)) => (Box::new(DeleteBookmark{}), value.into()),
        _ => {
            eprintln!("Error: No valid option specified.");
            // Consider providing usage information or exiting gracefully
            (Box::new(Help{}), "".into())
        }
    }
}

fn gen_options() -> Options {
    let mut options = Options::new();
    options.optopt("b", "database", "Database file to be used for bookmarks", "PATH");
    options.optopt("r", "remove-database", "Database file to be removed", "PATH");
    options.optopt("s", "save", "Save current path as BOOKMARK_NAME", "BOOKMARK_NAME");
    options.optopt("o", "show", "Show the path assigned to BOOKMARK_NAME", "BOOKMARK_NAME");
    options.optopt("d", "delete", "Delete BOOKMARK_NAME", "BOOKMARK_NAME");
    options.optflag("l", "list", "List all bookmarks in current DB");
    options.optflag("h", "help", "Print this help menu");
    options
}

pub fn print_usage(program: &Rc<str>, opts: &Options) {
    let brief = format!("Usage {} [options] \nWhen -s,-o,-d are passed in the same command, they take precedence -s > -o > -d", program);
    println!("{}", opts.usage(&brief));
}

