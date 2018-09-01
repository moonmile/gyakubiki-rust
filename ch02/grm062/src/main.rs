fn func( ch : char ) -> bool {
    if ch.is_alphabetic() || ch.is_digit(10) {
        true
    } else {
        false
    }
}

fn main() {
    println!("文字入力:");
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    // 最初の文字を取り出す
    let a = s.as_str().chars().nth(0).unwrap();
    // アルファベット、数値、その他を調べる
    if func(a) {
        println!("func が真(true)");
    } else {
        println!("func が偽(false)");
    }
}
