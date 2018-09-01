use std::fs::File;
use std::io::prelude::*;

fn main() {
    let fname = "src/main.rs";
    let mut f = File::open(fname).expect("file not found.");
    // 1バイト分のバッファ
    let mut buf = vec![0;1];    
    // 10文字読み込む
    for _ in 0..10 {
        f.read(&mut buf).expect("read error.");
        print!("{},", buf[0] as char );
    }
    println!("");
}
