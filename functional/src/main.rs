use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    cache: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            cache: HashMap::new(),
        }
    }

    fn value(&mut self, key: &u32) -> &u32 {
        let calculation = &self.calculation;
        self.cache.entry(*key).or_insert_with(|| calculation(*key))
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("First do {} pushups", expensive_result.value(&intensity));
        println!("Then do {} situps", expensive_result.value(&intensity));
    } else {
        if random_number == 3 {
            println!("Rest day");
        } else {
            println!("Run for {} minutes", expensive_result.value(&intensity));
        }
    }
}

fn main() {
    let simulated_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_specified_value, simulated_random_number);

    let x = 4;
    let equal_to_x = |z| x == z;
    let y = 4;
    assert!(equal_to_x(y));

    // let x = vec![1, 2, 3];
    // let equal_to_x = move |z| x == z;
    // println!("Can't use x here: {:?}", x);
    // let y = vec![1, 2, 3];
    // assert!(equal_to_x(y));
}
