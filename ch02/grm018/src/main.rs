// 引数がない関数
fn func1() {
    println!("in func1");
}
// 引数がある関数
fn func2( x: i32 ) {
    println!("in func2 {}", x);
}
// 戻り値がある関数
fn func3(x: i32) -> i32 {
    println!("in func3 {}", x );
    x + 10
}

fn main() {
    func1();
    func2(10);
    let ret = func3(10);
    println!("in main {}", ret );
}
