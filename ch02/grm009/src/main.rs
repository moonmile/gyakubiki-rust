fn main() {
    // 符号付きshort型
    let x1 : i16 = 10;
    let x2 = i16::max_value();
    let x3 = i16::min_value();
    println!("sizeof i16 is {}",  std::mem::size_of::<i16>());
    println!("x1: {}", x1 );
    println!("x2 max: {}", x2 );
    println!("x3 min: {}", x3 );

    // 符号なしshort型
    let y1 : u16 = 10;
    let y2 = u16::max_value();
    let y3 = u16::min_value();
    println!("sizeof u16 is {}", std::mem::size_of::<u16>());
    println!("y1: {}", y1 );
    println!("y2 max: {}", y2 );
    println!("y3 min: {}", y3 );
}
