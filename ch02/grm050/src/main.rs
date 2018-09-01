fn main() {
    println!("1文字入力する:");
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    // String型から最初の文字を取り出す
    let a = s.as_str().chars().nth(0).unwrap();
    // 否定で比較する
    if !a.is_alphabetic() {
        println!("{} はアルファベットではない", a );
    } else {
        println!("{} はアルファベットである", a );
    }
}
