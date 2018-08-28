fn main() {
    // 符号無しint型
    let x1 : u32 = 10;
    let x2 = u32::max_value();
    let x3 = u32::min_value();
    println!("sizeof u32 is {}",  std::mem::size_of::<u32>());
    println!("x1: {} 0x{:X}", x1, x1 );
    println!("x2 max: {} 0x{:X}", x2, x2 );
    println!("x3 min: {} 0x{:X}", x3, x3 );
}
