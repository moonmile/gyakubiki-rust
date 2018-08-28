fn main() {
    // プラス演算子
    let m = 10 ;
    let n = 20 ;
    // 数値の加算
    println!("m + n = {}", m + n );
    // 文字列の結合
    let s1 = "Hello ";
    let s2 = "Rust ";
    let s3 = "World.";
    // 最初だけ &str にしておく
    let s = s1.to_string() + s2 + s3 ;
    println!("s1 + s2 + s3 = {}", s );
}
