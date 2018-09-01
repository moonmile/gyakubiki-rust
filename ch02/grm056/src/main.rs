struct Point {
    x: i32,
    y: i32,
}

fn func( pt : &Point ) -> String {
    format!("x:{} y:{}", pt.x, pt.y )
}

fn main() {

    let pt = Point { x:10, y:20 };
    // 通常のメソッド関数の呼び出し
    println!("{}", func(&pt));
    // 関数ポインタを使う
    let f = func ;
    println!("{}", f( &pt ));
    // クロージャを使う
    let g : fn(&Point) -> String = |pt| format!("x:{} y:{}", pt.x, pt.y ) ;
    println!("{}", g( &pt ));
    // impl した関数ポインタは扱えない？
}
