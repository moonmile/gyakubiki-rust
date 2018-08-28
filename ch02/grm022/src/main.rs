fn main() {
    // 配列の場合
    let v = [1,2,3,4,5,6,7,8,9,10];
    for n in v.iter() {
        print!("{},", n );
    }
    println!("");
    // 添え字でもできるが、あまり意味がない
    for i in 0 .. v.len() {
        print!("{},", v[i] );
    }
    println!("");

    // ベクトルの場合
    let v = vec![1,2,3,4,5,6,7,8,9,10];
    // イテレータの扱いが配列と少し違う
    for n in v {
        print!("{},", n );
    }
}
