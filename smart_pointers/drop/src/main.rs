struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn main() {
    let a = CustomSmartPointer { data: String::from("eh") };
    let b = CustomSmartPointer { data: String::from("bee") };
    println!("CustomSmartPointers created");
    drop(a);
    println!("Dropped before going out of scope");
}
