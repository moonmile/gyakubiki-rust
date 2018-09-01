fn main() {
    let ary = [0,1,2,3,4,5,6,7,8,9];
    for x in &ary {
        print!("{},", x );
    }
    println!("");
    let ary = vec![0,1,2,3,4,5,6,7,8,9];
    for x in ary {
        print!("{},", x );
    }
    println!("");
    // ベクタを自前で作る
    let mut ary = vec![];
    for i in 0..10 {
        ary.push( i );
    }
    for x in ary {
        print!("{},", x );
    }
    println!("");
}
