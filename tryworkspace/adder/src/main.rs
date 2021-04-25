extern crate add_one;
extern crate rand;
extern crate add_two;


fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
    println!("{} plus random is {}!", num, add_one::add_rand(num));
    println!("new random value is {}!", rand::random::<i32>());
    println!("Hello, world! {} plus two is {}!", num, add_two::add_two(num));
}
