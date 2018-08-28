fn main() {
    // 比較演算子
    let x = 10 ;
    let y = 10 ;
    println!("x:{} y:{}", x, y );
    if x != y {
       println!("x と y は等しくない"); 
    } else {
       println!("x と y は等しい"); 
    }

    let y = 20 ;
    println!("x:{} y:{}", x, y );
    if x != y {
       println!("x と y は等しくない"); 
    } else {
       println!("x と y は等しい"); 
    }

    // if式の戻り値を表示する
    let s = 
        if x == y { 
            "x と y は等しい" 
        } else { 
            "x と y は等しくない" 
        };
    println!("{}", s );
}
