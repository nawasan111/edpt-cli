use std::io::{stdin, stdout, Write};
use endecript::{cli::checker, vigenere};

fn main() {
    checker();
    return;
    vigenere::table();
    let mut buf = String::new();

    // input
    print!("input (text) :");
    let _ = stdout().flush();
    let _ = stdin().read_line(&mut buf);
    let text = buf.trim().to_uppercase();
    let mut buf = String::new();

    print!("input (key) :");
    let _ = stdout().flush();
    let _ = stdin().read_line(&mut buf);
    let key = buf.trim().to_ascii_uppercase();
    // end input
    let mut key_l = "".to_string();
    while text.len() > key_l.len() {
        key_l = format!("{}{}", key_l, key);
    }

    let text_vec = text.as_bytes().to_vec();
    let key_vec = key_l.as_bytes().to_vec();

    for i in 0..text_vec.len() {
        let r = vigenere::decript(text_vec[i], key_vec[i]);
        print!("{}", r);
    }
    println!();

    return;
}
