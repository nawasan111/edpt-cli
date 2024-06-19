use std::collections::HashMap;

pub fn table() {
    for i in 1..27_u8 {
        for j in 0..27_u8 {
            let mut num = i + j;
            if num > 26 {
                num -= 26;
            }
            print!("{} ", (num + 64) as char);
        }
        println!();
    }
}

pub fn decript(in1: u8, in2: u8) -> char {
    let in1 = in1 - 65;
    let in2 = in2 - 65;

    let num = if in1 >= in2 {
        in1 - in2
    } else {
        26 - in2 + in1
    };
    (num + 65) as char
}

pub fn encript(in1: u8, in2: u8) -> char {
    let mut num = (in1 - 65) + (in2 - 64);
    if num > 26 {
        num -= 26;
    }
    (num + 64) as char
}

pub fn vi_cli(command: &Vec<String>, _: &Vec<String>, value: &HashMap<&str, String>) {
    let action = match command.get(1) {
        Some(v) => v.trim().to_lowercase(),
        None => panic!("Not found action command"),
    };

    if action == "table" {
        table();
        return;
    }

    let message = match command.get(2) {
        Some(v) => v.trim().to_uppercase(),
        None => panic!("Not found message for action"),
    };
    let key = match value.get("key") {
        Some(v) => v.trim().to_uppercase(),
        None => panic!("Not found key"),
    };
    let mut key_fill = String::new();
    while message.len() > key_fill.len() {
        key_fill = format!("{}{}", key_fill, key);
    }

    let len = message.len();
    let message = message.as_bytes().to_vec();
    let key_fill = key_fill.as_bytes().to_vec();

    match action.as_str() {
        "en" => {
            let mut result = String::new();
            for i in 0..len {
                result = format!("{}{}", result, encript(message[i], key_fill[i]));
            }
            println!("{}", result);
        }
        "de" => {
            let mut result = String::new();
            for i in 0..len {
                result = format!("{}{}", result, decript(message[i], key_fill[i]));
            }
            println!("{}", result);
        }
        _ => println!("Action not currect"),
    }
}
