fn main() {
    // 符号付きint型
    let x1 : i32 = 10;
    let x2 = i32::max_value();
    let x3 = i32::min_value();
    println!("sizeof i32 is {}",  std::mem::size_of::<i32>());
    println!("x1: {}", x1 );
    println!("x2 max: {}", x2 );
    println!("x3 min: {}", x3 );

    // 符号なしint型
    let y1 : u32 = 10;
    let y2 = u32::max_value();
    let y3 = u32::min_value();
    println!("sizeof u32 is {}", std::mem::size_of::<u32>());
    println!("y1: {}", y1 );
    println!("y2 max: {}", y2 );
    println!("y3 min: {}", y3 );
}
