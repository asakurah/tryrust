
enum Fluit {
    Apple,
    Orange,
    Pine,
}


fn print_fluit(val: Fluit) {
    match val {
        Apple => println!("Apple"),
        Orange => println!("Orange"),
        Pine => println!("Pine"),
    }
}


fn main() {
    print_fluit(Fluit::Apple);
    print_fluit((1 as isize) as Fluit);
}
