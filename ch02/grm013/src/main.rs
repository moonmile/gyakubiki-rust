fn main() {
    // 数値double型
    let x1 : f64 = 100.234;
    let x2 = std::f64::MAX;
    let x3 = std::f64::MIN;
    println!("sizeof f64 is {}",  std::mem::size_of::<f64>());
    println!("x1: {}", x1 );
    println!("x2 max: {:e}", x2 );
    println!("x3 min: {:e}", x3 );
}
