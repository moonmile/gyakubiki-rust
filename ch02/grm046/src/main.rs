fn main() {
    let x : u32 = 0xFFFF0000;
    let y : u32 = 0xFF00FF00;
    println!("x     = {:X}", x );
    println!("y     = {:X}", y );
    println!("x ^ y = {:08X}", x ^ y );
}
