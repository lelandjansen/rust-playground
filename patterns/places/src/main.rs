fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Coordinates: ({}, {})", x, y);
}
fn main() {
    for x in 1..=5 {
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("{}", x),
        }
    }
    println!();

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "21".parse();
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background color",
                 color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age < 30 {
            println!("Using orange as the background color");
        } else {
            println!("Using magenta as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
    println!();

    let mut stack = Vec::new();
    for x in 1..=3 {
        stack.push(x);
    }
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    println!();

    let v = vec![1, 2, 3];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    println!();

    let z = 3;
    let (x, y, z) = (1, 2, z);
    let point = (x, y + z);
    print_coordinates(&point);



}
