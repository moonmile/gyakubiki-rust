fn main() {
    // １ずつ増加させる
    for i in 0..10 {
        print!("{},", i);
    }
    println!("");
    // 1ずつ減少させる、ことはできない
    // 2ずつ増加させる、こともできない
    // 普通に2ずつ加算する
    let mut v = 0 ;
    for _i in 0..10 {
        v += 2;
        print!("{},", v);
    }
    println!("");
}
