use std::fs::File;
use std::io;
use std::io::Read;
// use std::io::ErrorKind;

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut file = match File::open("username.txt") {
    //     Ok(file) => file,
    //     Err(error) => return Err(error),
    // };
    // let mut username = String::new();
    // match file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(error) => Err(error),
    // }
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(error) => panic!("Error: {:?}", error),
    }

    // let file_name = "something.txt";
    // // let _file = File::open(file_name).unwrap();
    // let _file = File::open(file_name).expect("Could not open file");

    // let _file = match File::open(file_name) {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //         match File::create(file_name) {
    //             Ok(file) => file,
    //             Err(error) => panic!("Error creating file: {:?}", error),
    //         }
    //     },
    //     Err(error) => panic!("Error opening file: {:?}", error),
    // };

    // let v = vec![1, 2, 3];
    // println!("{}", v[99]);
    // panic!("Crash and burn");
}
