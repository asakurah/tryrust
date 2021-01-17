pub struct MyObj {
    msg: String,
}


impl MyObj {
    pub fn new(msg: &str) -> MyObj {
        MyObj{
            msg: String::from(msg),
        }
    }
}


impl Drop for MyObj {
    fn drop(&mut self) {
        println!("message: {}", self.msg);
    }
}

