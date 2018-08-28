fn main() {
    // 符号付きlong型
    let x1 : i64 = 10;
    let x2 = i64::max_value();
    let x3 = i64::min_value();
    println!("sizeof i64 is {}",  std::mem::size_of::<i64>());
    println!("x1: {}", x1 );
    println!("x2 max: {}", x2 );
    println!("x3 min: {}", x3 );

    // 符号なしlong型
    let y1 : u64 = 10;
    let y2 = u64::max_value();
    let y3 = u64::min_value();
    println!("sizeof u64 is {}", std::mem::size_of::<u64>());
    println!("y1: {}", y1 );
    println!("y2 max: {}", y2 );
    println!("y3 min: {}", y3 );
}
