fn main() {
    // ループの色々
    // 即値を使ってループ
    let mut v = 0 ;
    for i in 0..10 {
        v += i ;
    }
    println!("合計値は {}", v );
    // 最大値に変数を使う
    let max = 10 ;
    v = 0;
    for i in 0..max {
        v += i ;
    }
    println!("合計値は {}", v );
    // 配列の添え字で使う
    let arr = [0,1,2,3,4,5,6,7,8,9];
    v = 0;
    for i in 0..arr.len() {
        v += arr[i] ;
    }
    println!("合計値は {}", v );
    // 普通はこっちを使う
    v = 0;
    for it in arr.iter() {
        v += it ;
    }
    println!("合計値は {}", v );
    // & を使ってイテレータを使う
    v = 0;
    for it in &arr {
        v += it ;
    }
    println!("合計値は {}", v );
    // ベクタの場合も同じ
    let arr = vec![0,1,2,3,4,5,6,7,8,9] ;
    v = 0;
    for it in arr {
        v += it ;
    }
    println!("合計値は {}", v );
}
