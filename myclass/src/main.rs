mod myclass;
use myclass::MyClass;

fn main() {
    println!("Hello, world!");

    let name = String::from("hoge");
    let age = 8;
    let m = MyClass::new(&name, age);
    println!("Hello, {}", m);
}

