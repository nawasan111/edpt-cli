/*
 * text = hello world
 * key{number}  = 123456 / ห้ามซ้ำ
 */

use std::collections::HashMap;

fn key_to_int(key: String) -> Option<Vec<usize>> {
    if key.len() > 9 {
        return None;
    }
    let key_array = key
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .map(|x| match x.to_string().parse::<usize>() {
            Ok(v) => Some(v),
            Err(_) => None,
        })
        .collect::<Option<Vec<usize>>>();

    let key_array = match key_array {
        Some(v) => v,
        None => {
            return None;
        }
    };
    // valid key
    let key_len = key_array.len();

    for i in 1..key_len + 1 {
        let mut is_in = false;
        for j in key_array.iter() {
            if i == *j {
                is_in = true;
                break;
            }
        }
        if !is_in {
            return None;
        }
    }

    Some(key_array)
}

fn encript(text: String, key: String) -> Option<String> {
    let mut enc_text = "".to_string();
    let key_len = key.len();
    let text_len = text.len();

    let loop_time = if text_len % key_len != 0 {
        text_len / key_len + 1
    } else {
        text_len / key_len
    };

    let text_array = text.chars().collect::<Vec<char>>();
    let key_array = match key_to_int(key) {
        Some(v) => v,
        None => {
            println!("key not correct");
            return None;
        }
    };
    for i in 0..loop_time {
        for j in 0..key_len {
            let index = *key_array.get(j).expect("key error") - 1;
            enc_text = format!(
                "{}{}",
                enc_text,
                text_array.get((i * key_len) + index).unwrap_or(&'#')
            );
        }
    }
    Some(enc_text)
}

pub fn co_cli(command: &Vec<String>, _: &Vec<String>, value: &HashMap<&str, String>) {
    let text = match command.get(1) {
        Some(v) => v.to_lowercase(),
        None => {
            println!("not found message for action");
            return;
        }
    };
    let key = match value.get("key") {
        Some(v) => v.to_lowercase(),
        None => panic!("Not found key"),
    };
    if let Some(msg) = encript(text, key) {
        println!("{}", msg);
    }
}
