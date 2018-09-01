fn main() {
    let s = "Hello Rust.";
    // ポインタというか &str を一文字ずつ char で取り出す
    for c in s.chars() {
        print!("{} ", c );
    }
    println!("");
}
