pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn help() {
    println!(
        r#"
EDPT - command line program to encript or decript message

Usage:
    edpt [mode] [action] <option> [inpu]

Mode:
    vi              Vigenere, encript or decript message with key.
    co              Columnar Transposition, Switch column positions.
                    this mode has no 'action'.

Action:
    en              encript
    de              decript

Option:
    -k{{value}}     key for encript or decript.
                    some action needed.

Example:
    edpt vi en -khello Helloworld
    edpt co -khello Helloworld

"#
    );
}
