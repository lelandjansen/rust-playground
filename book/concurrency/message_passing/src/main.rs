use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx1, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx1);
    thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("from"),
            String::from("anoher"),
            String::from("thread"),
        ];
        for value in values {
            tx1.send(value).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let values = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for value in values {
            tx2.send(value).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }
}
