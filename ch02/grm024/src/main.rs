fn main() {
    // 文字列の配列
    let v = ["Rust", "C++", "Scala", "F#"];
    for it in v.iter() {
        println!("{}", it );
    }
    // 文字列のベクター
    let v = vec!["Rust", "C++", "Scala", "F#"];
    for it in v {
        println!("{}", it );
    }
}

