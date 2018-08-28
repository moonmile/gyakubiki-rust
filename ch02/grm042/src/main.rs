fn main() {
    let mut m = 10 ;
    m += 20 ;
    println!("m = {}", m );

    let mut s = "".to_string();
    s += "Hello ";
    s += "Rust ";
    s += "World.";
    println!("{}", s );
}
