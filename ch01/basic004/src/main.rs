struct A {
    lang: String,
}

impl A {
    fn new( lang: &str ) -> A {
        A { 
            lang: lang.to_string(),
        }
    }
    fn print(&self) {
        println!("Hello {} world, again.", self.lang);
    }
}

fn main() {
    println!("Hello, world!");
    let a = A::new("Rust");
    a.print();
}
