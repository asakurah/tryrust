use std::fmt;


pub struct MyClass {
    name: String,
    age: u8,
}


impl fmt::Display for MyClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.name, self.age)
    }
}


impl MyClass {
    pub fn new(name: &String, age: u8) -> MyClass {
        MyClass {
            name: name.clone(),
            age,
        }
    }
}
