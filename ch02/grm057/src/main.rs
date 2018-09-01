fn main() {
    let n = 1 ;
    if n == 0 {
        println!("n is zero.");
    } else {
        println!("n is not zero.");
    }
    // if を式として扱う
    println!("n is {}.", 
        if n == 0 { "zero" } else { "not zero" } );
}
