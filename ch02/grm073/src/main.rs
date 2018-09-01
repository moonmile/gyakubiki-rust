fn main() {
    for i in 0..10 {
        // 偶数の場合は以下の処理を飛ばす
        if i % 2 == 0 { continue; }
        println!("{}", i );
    }
}
