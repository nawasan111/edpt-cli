use std::{collections::HashMap, env::args};

use crate::vigenere::vi_cli;
pub fn checker() {
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

    if option.len() > 0 {
        match option.get(0) {
            Some(opt) => match opt.as_str() {
                "-h" | "--help" => println!("help"),
                _ => (),
            },
            None => (),
        }
    }

    if command.len() > 0 {
        match command[0].as_str() {
            "vi" => {
                vi_cli(&command, &option, &value);
            }
            _ => println!("command not found"),
        }
    }
}
