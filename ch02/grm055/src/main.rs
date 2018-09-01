struct Point {
    x: i32,
    y: i32,
}

struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn to_string(&self) -> String {
        format!("{} ({})", self.name, self.age )
    }
}

fn func( pt : &Point ) -> String {
    format!("x:{} y:{}", pt.x, pt.y )
}

fn main() {
    let pt = Point { x:10, y:20 };
    println!("{}", func(&pt));
    
    let me = Person { name: "masuda".to_string(), age: 50 };
    println!("{}", me.to_string());
}
