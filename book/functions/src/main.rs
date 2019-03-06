//fn stuff(x: f64) {
//    println!("{} is a float", x);
//}

fn stuff(x: i32) {
    println!("{} is an i32", x);
}

fn main() {
    println!("Hello, world!");
    let x: i32 = 12;
    stuff(x);
    let x = {
        let x = 7;
        println!("ohai");
        x * 3
    };
    println!("Here");
    let y = x / 3;
    println!("y = {}", y);


    let mut n: i8 = 0;
    loop {
        println!("n: {}", n);
        n += 1;
    }
}
