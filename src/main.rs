extern crate libusb;
extern crate uinput;

extern crate log;
extern crate simple_logger;

extern crate failure;

extern crate toml;
extern crate serde;
extern crate serde_derive;

mod g13_manager;
mod g13_device;
mod g13_key;
mod constants;
mod config;

use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::io::prelude::*;
use std::process::exit;

use log::{warn, error};

use crate::g13_manager::G13Manager;
use crate::config::Configuration;

fn read_config(filename: &str) -> Result<Configuration, failure::Error> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                warn!("No config file found, using default configuration");
                return Ok(Configuration::default());
            },
            _ => {
                return Err(err.into());
            }
        }
    };
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;

    Ok(toml::from_str(content.as_str())?)
}

fn main() {
    // enable logging
    simple_logger::init().unwrap();

    let configuration = match read_config("config.toml") {
        Ok(config) => config,
        Err(err) => {
            error!("Encountered error reading the config file: {}", err);
            exit(1);
        }
    };

    for (key, val) in configuration.actions.iter() {
        println!("Key: {} Val: {}", key, val);
    }

    let manager = G13Manager::new().unwrap();
    manager.mainloop();
}
