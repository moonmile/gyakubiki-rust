use std::fs::File;
use std::io::prelude::*;

fn main() {
    let fname = "sample.txt";
    let mut f = File::open(fname).expect("file not found.");
    let mut line = String::new();
    f.read_to_string(&mut line).expect("read error.");
    println!("read text\n{}", line);
}
