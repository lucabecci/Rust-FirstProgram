use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main(){

    //vars
    let stdout = stdout();
    let message = String::from("Hello world of my Rust app by Luca Becci");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

