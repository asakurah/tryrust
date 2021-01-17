mod mymod1;
mod mymod2;

fn main() {
    println!("Hello, world!");

    mymod1::myfn1();
    mymod2::myfn1();
    mymod2::mymod1::myfn1();
}

