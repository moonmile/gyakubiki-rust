fn main() {
    let s = "Hello Rust World.";
    for (i,ch) in s.chars().enumerate() {
        println!("{}: {}", i, ch );
    }
}
