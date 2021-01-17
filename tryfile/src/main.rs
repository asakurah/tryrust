mod readtostr;

fn main() {
    println!("Hello, world!");
    readtostr::begin("test.txt");
    readtostr::begin("unexists.txt");
}
