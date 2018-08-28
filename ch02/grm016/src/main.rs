fn main() {
    // bool型
    let x1 = true;
    let x2 = false;
    println!("sizeof u32 is {}",  std::mem::size_of::<bool>());
    println!("x1: {}", x1 );
    println!("x2: {}", x2 );
}
