fn main() {
    // 符号付きint型
    let x1 : i32 = 10;
    let x2 = i32::max_value();
    let x3 = i32::min_value();
    println!("sizeof i32 is {}",  std::mem::size_of::<i32>());
    println!("x1: {} 0x{:X}", x1, x1 );
    println!("x2 max: {} 0x{:X}", x2, x2 );
    println!("x3 min: {} 0x{:X}", x3, x3 );
}
