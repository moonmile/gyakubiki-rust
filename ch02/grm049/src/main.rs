fn main() {
    println!("2文字入力する:");
    let mut s = String::new();
    std::io::stdin().read_line(buf)(&mut s);
    // String型から最初と2番めの文字を取り出す
    let a = s.as_str().chars().nth(0).unwrap();
    let b = s.as_str().chars().nth(1).unwrap();
    // 論理和で比較する
    if a.is_alphabetic() || b.is_alphabetic() {
        println!("{} {} はどちらかがアルファベット", a, b );
    } else {
        println!("{} {} はその他の文字を含む", a, b );
    }
}