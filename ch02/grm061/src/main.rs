fn main() {
    println!("文字入力:");
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    // 最初の文字を取り出す
    let a = s.as_str().chars().nth(0).unwrap();
    // アルファベット、数値、その他を調べる
    if a.is_alphabetic() {
        println!("{} はアルファベット", a );
    } else if a.is_digit(10) {
        println!("{} は数値", a );
    } else {
        println!("{} はその他の文字", a );
    }
}
