// グローバル変数は大文字で指定する
static X : i32 = 10 ;
fn main() {
    // ローカル変数は小文字で指定する
    let x = 20 ;
    println!("local x = {}", x );
    println!("global X = {}", X );
}
