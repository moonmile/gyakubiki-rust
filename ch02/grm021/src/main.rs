// イテレータを渡して値を変える
fn func1() {
    // ???
}
// 前後のイテレータを渡して値を変える
fn func2() {
    // ???
}

fn main() {
    let mut v = [1,2,3,4,5,6,7,8,9,10];
    for n in v.iter() {
        print!("{},", n );
    }
    println!("");

    v[9] = 99 ;
    for n in v.iter() {
        print!("{},", n );
    }
    println!("");

    for i in 0 .. v.len() {
        v[i] = v[i] + 1 ;
    }
    for n in v.iter() {
        print!("{},", n );
    }
    println!("");
}
