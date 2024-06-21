use std::{collections::HashMap, env::args};

use crate::{
    columnar::co_cli,
    help::{help, VERSION},
    vigenere::vi_cli,
};
pub fn cli_system() {
    let mut command: Vec<String> = Vec::new();
    let mut option: Vec<String> = Vec::new();
    let mut value: HashMap<&str, String> = HashMap::new();

    for input in args() {
        if input.len() < 2 {
            continue;
        }
        let mut a: Vec<char> = input.chars().collect();
        if a[0] == '-' {
            if a[1] == 'K' || a[1] == 'k' {
                a.remove(0);
                a.remove(0);
                value.insert("key", a.iter().collect::<String>());
            } else {
                option.push(input.clone());
            }
        } else {
            command.push(input.clone());
        }
    }
    command.remove(0);

    if let Some(opt) = option.get(0) {
        match opt.as_str() {
            "-h" | "--help" => {
                help();
                return;
            }
            "-v" | "--version" => {
                println!("v{}", VERSION);
                return;
            }
            _ => (),
        }
    }
    if let Some(cmd) = command.get(0) {
        match cmd.as_str() {
            "co" => {
                co_cli(&command, &option, &value);
            }
            "vi" => {
                vi_cli(&command, &option, &value);
            }
            _ => println!("command not found"),
        }
    }
    if command.len() == 0 && option.len() == 0 {
        help();
    }
}
