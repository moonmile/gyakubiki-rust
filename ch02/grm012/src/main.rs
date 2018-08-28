fn main() {
    // 数値short型
    let x1 : f32 = 100.234;
    let x2 = std::f32::MAX;
    let x3 = std::f32::MIN;
    println!("sizeof f32 is {}",  std::mem::size_of::<f32>());
    println!("x1: {}", x1 );
    println!("x2 max: {:e}", x2 );
    println!("x3 min: {:e}", x3 );
}
