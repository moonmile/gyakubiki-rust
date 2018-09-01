fn main() {
    let mut ch = 'A' ;
    for i in 0..10 {
        println!("{}: {}", i, ch );
        ch = ((ch as u8) + 1) as char ;
    }
}
