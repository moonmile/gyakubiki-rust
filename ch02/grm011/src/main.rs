fn main() {
    // char型
    let x1 : char = 'R';
    let x2 = std::char::MAX;
    println!("sizeof char is {}",  std::mem::size_of::<char>());
    println!("x1: {}", x1 );
    println!("x2 max: {}", x2 );

    // 符号付きi8型
    let y1 : i8 = 10;
    let y2 = i8::max_value();
    let y3 = i8::min_value();
    println!("sizeof i8 is {}", std::mem::size_of::<i8>());
    println!("y1: {}", y1 );
    println!("y2 max: {}", y2 );
    println!("y3 min: {}", y3 );

    // 符号なしu8型
    let y1 : u8 = 10;
    let y2 = u8::max_value();
    let y3 = u8::min_value();
    println!("sizeof u8 is {}", std::mem::size_of::<u8>());
    println!("y1: {}", y1 );
    println!("y2 max: {}", y2 );
    println!("y3 min: {}", y3 );
}
