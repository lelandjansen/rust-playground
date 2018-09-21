use std::slice;

static HELLO_WORLD: &str = "Hello, World!";
static mut COUNTER: u32 = 0;

extern "C" {
    fn abs(input: i32) -> i32;
}

unsafe fn dangerous() {}

// Possible implementation
fn _split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    assert!(mid < slice.len());
    let ptr = slice.as_mut_ptr();
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(
                ptr.offset(mid as isize),
                slice.len() - mid,
            ),
        )
    }
}

fn add_to_count(increment: u32) {
    unsafe {
        COUNTER += increment;
    }
}

unsafe trait Foo {}
unsafe impl Foo for i32 {}

fn main() {
    println!("Hello, world!");
    let mut number = 5;
    let r1 = &number as *const i32;
    let r2 = &mut number as *mut i32;

    let address = 0x012345usize;
    let _r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("abs(-3) according to C = {}", abs(-3));
    }

    println!("{}", HELLO_WORLD);

    add_to_count(3);
    unsafe {
        println!("COUNTER = {}", COUNTER);
    }
}
