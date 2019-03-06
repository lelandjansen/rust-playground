fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let second = x.1;
    let (first, _, third) = x;
    println!("first: {}, second: {}, third: {}", first, second, third);
    let a = [1, 2, 3, 4, 5];
    for item in a.iter() {
        print!("{} ", item);
    }
    println!();
}
