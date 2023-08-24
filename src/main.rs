use argparse::{ArgumentParser, Store, StoreTrue};
use std::{env, fs::File, io::Read, process::exit};
mod macros;

// define package metadata
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const REPO: &str = env!("CARGO_PKG_REPOSITORY");

fn main() {
    let (input_file_str, key) = parse_args();
    debug!("input file: {input_file_str}");
    debug!("key: {key}");

    let input_file_path = std::path::Path::new(&input_file_str);
    if !input_file_path.exists() || !input_file_path.is_file() {
        error!("{input_file_str} file does not exist");
        exit(1);
    }

    let mut input_file = match File::open(&input_file_str) {
        Ok(f) => f,
        Err(e) => {
            error!("error opening {input_file_str}: {e}");
            exit(5);
        }
    };

    // read file content to string
    let mut input_file_contents = String::new();
    match input_file.read_to_string(&mut input_file_contents) {
        Ok(_) => (),
        Err(e) => {
            error!("error reading {input_file_str}: {e}");
            exit(5);
        }
    };

    // parse TOML content
    let toml_content = match input_file_contents.parse::<toml::Value>() {
        Ok(t) => t,
        Err(e) => {
            error!("error parsing {input_file_str}: {e}");
            exit(5);
        }
    };

    debug!("toml_content: {:?}", toml_content);

    // split key into parts
    let key_parts: Vec<String> = key.split('.').map(|s| s.to_string()).collect();

    // find value for key
    let value = find_value(key_parts, toml_content).unwrap();
    debug!("returned value: {:?}", value);

    // print value
    match_and_print(value, false);
}

fn print_array(array: toml::value::Array) {
    // function for printing each value in an array
    for value in array {
        match_and_print(value, true);
    }
}

fn match_and_print(value: toml::Value, from_array: bool) {
    match value {
        toml::Value::String(s) => println!("{s}"),
        toml::Value::Integer(i) => println!("{i}"),
        toml::Value::Float(f) => println!("{f}"),
        toml::Value::Boolean(b) => println!("{b}"),
        toml::Value::Datetime(d) => println!("{d}"),
        toml::Value::Array(a) => print_array(a),
        toml::Value::Table(_t) => {
            if from_array {
                error!("table in an array? how did you do this?\nplease open an issue at {REPO}");
            } else {
                error!("value is a table");
            }
            exit(6);
        }
    }
}

fn find_value(key_parts: Vec<String>, mut toml_content: toml::Value) -> Option<toml::Value> {
    for (i, part) in key_parts.iter().enumerate() {
        debug!("part: {part}");
        if toml_content.is_table() {
            let table = toml_content.as_table().unwrap();
            // if key is in table and is the last part of the key, return the value
            if table.contains_key(part) {
                debug!("found key: {part}");
                let value = table.get(part).unwrap();
                debug!("value: {:?}", value);
                if i == key_parts.len() - 1 {
                    return Some(value.clone());
                }
                // if key is in table and is not the last part of the key
                // set toml_content to the value
                toml_content = value.clone();
            } else {
                error!("key not found: {part}");
                exit(1);
            }
        } else {
            error!("value does not seem to be a TOML table");
            exit(1);
        }
    }
    None
}

fn parse_args() -> (String, String) {
    // parse command line arguments
    let mut input_file = String::new();
    let mut key = String::new();
    let mut debug: bool = false;
    let mut version: bool = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("rtoml - read a TOML file and print the value of a key");
        ap.refer(&mut input_file)
            .add_argument("input_file", Store, "");
        ap.refer(&mut key).add_argument("key", Store, "");
        ap.refer(&mut debug)
            .add_option(&["--debug"], StoreTrue, "enable debug logging");
        ap.refer(&mut version).add_option(
            &["-V", "--version"],
            StoreTrue,
            "print version and exit",
        );
        ap.parse_args_or_exit();
    }
    if version {
        println!("rtoml v{VERSION}");
        println!("author: {AUTHORS}");
        println!("source/documentation at {REPO}");
        exit(0);
    }
    // init logging
    if debug {
        env::set_var("RUST_LOG", "debug");
        env::set_var("RUST_BACKTRACE", "1");
    } else {
        env::set_var("RUST_LOG", "info");
    }
    debug!("debug logging enabled");

    if input_file.is_empty() && key.is_empty() {
        error!("input file and key are required");
        exit(2);
    } else if input_file.is_empty() {
        error!("input file is required");
        exit(2);
    } else if key.is_empty() {
        error!("key is required");
        exit(2);
    }

    (input_file, key)
}
