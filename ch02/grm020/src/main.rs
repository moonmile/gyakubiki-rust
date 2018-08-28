fn func( v: &mut i32 ) {
    // 値を変更する
    *v = 10 ;
}

fn main() {
    let mut n = 0;
    println!("n: {}", n );
    func( &mut n );
    println!("n: {}", n );
}
