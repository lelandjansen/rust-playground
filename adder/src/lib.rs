#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 { a + 2 }

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    //String::from("Ohai")
}

pub struct Guess {
    pub value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || 100 < value {
            panic!("Guess value must be in the range [1, 100], got {}.", value);
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Oh noes!");
    // }
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 3 };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 3 };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let name = "Leland";
        assert!(greeting(name).contains(name));
    }

    #[test]
    fn greeting_contains_name_descriptive() {
        let name = "Leland";
        let result = greeting(name);
        assert!(result.contains(name),
        "Greeting did not contain name, result was `{}`", result);
    }

    #[test]
    #[should_panic(expected = "Guess value must be in the range [1, 100]")]
    fn guess_0() {
        Guess::new(0);
    }

    #[test]
    #[should_panic(expected = "Guess value must be in the range [1, 100]")]
    fn guess_over_100() {
        Guess::new(200);
    }
}
