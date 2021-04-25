extern crate rand;


pub fn add_one(x: i32) -> i32 {
    x + 1
}


pub fn add_rand(x: i32) -> i32 {
    x + rand::random::<i32>()
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_one(7), 8);
    }


    #[test]
    fn it_works_rand() {
        let x = add_rand(10);
        assert_eq!(x, x);
    }
}
