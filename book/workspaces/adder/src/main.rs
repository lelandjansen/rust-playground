extern crate add_one;

fn main() {
    let number = 2;
    println!("{} + 1 = {}", number, add_one::add_one(number));
}
