fn main() {
    let x = 2;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got fifty"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        1...5 => println!("one through five"),
        _ => println!("anything"),
    }

    let x = 'c';
    match x {
        'A'...'Z' => println!("Upperase"),
        'a'...'z' => println!("Lowercase"),
        _ => println!("anything"),
    }

    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the y-axis at x = {}", x),
        Point { x: 0, y } => println!("On the x-axis at y = {}", y),
        Point { x, y } => println!("Not on an axis at ({}, {})", x, y),
    }

    enum Message {
        _Quit,
        _Move { x: i32, y: i32 },
        _Write(String),
        ChangeColor(u8, u8, u8),
    }
    let message = Message::ChangeColor(0, 160, 255);
    match message {
        Message::_Quit => println!("Quit has not data to destructure"),
        Message::_Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::_Write(text) => println!("Message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Chagne color to red = {}, green = {}, blue = {}", r, g, b)
        },
    }

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];
    let sum_of_squres: i32 = points
        .iter()
        .map(|&Point { x, y }| x * x + y * y)
        .sum();
    println!("sum of squares: {}", sum_of_squres);

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet = {}, inches = {}, x = {}, y = {}", feet, inches, x, y);

    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    struct Point3d {
        x: i32,
        _y: i32,
        _z: i32,
    }
    let origin = Point3d { x: 0, _y: 0, _z: 0 };
    match origin {
        Point3d { x, .. } => println!("x = {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => println!("numbers: {} to {}", first, last),
    }

    // ambiguous
    //match numbers {
    //    (.., second, ..) => println!("second: {}", second),
    //}
    
    let robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }
    println!("robot name is: {:?}", robot_name);

    let mut robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref mut name) => *name = String::from("Antoher name"),
        None => (),
    }
    println!("robot name is: {:?}", robot_name);

    let number = Some(4);
    match number {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    enum HelloMessage {
        Hello { id: i32 },
    }
    let message = HelloMessage::Hello { id: 5 };
    match message {
        HelloMessage::Hello { id: id_variable @ 3...7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        HelloMessage::Hello { id: 10...12 } => {
            println!("Found an id in another range")
        },
        HelloMessage::Hello { id } => println!("Found some other id: {}", id),
    }
}
