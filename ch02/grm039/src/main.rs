fn main() {
    let x = 10;
    // 代入演算子
    let mut m = x ;
    println!("x:{} m:{}", x, m );
    // 同じ値を連続で代入...することはできない
    // let n = m = 20 ;
    m = 20 ;
    let n = m ;
    println!("m:{} n:{}", m, n );

}
