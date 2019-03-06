use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "Leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "Branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        println!(
            "Leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "Leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

//use std::rc::Rc;
//use std::cell::RefCell;
//use List::{Cons, Nil};
//
//#[derive(Debug)]
//enum List {
//    Cons(i32, RefCell<Rc<List>>),
//    Nil,
//}
//
//impl List {
//    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//        match *self {
//            Cons(_, ref item) => Some(item),
//            Nil => None,
//        }
//    }
//}
//
//fn main() {
//    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
//    println!("a initial rc count: = {}", Rc::strong_count(&a));
//    println!("a next item = {:?}", a.tail());
//    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
//    println!("a rc count after creating b: = {}", Rc::strong_count(&a));
//    println!("b initial rc count: = {}", Rc::strong_count(&b));
//    println!("b next item = {:?}", b.tail());
//    if let Some(link) = a.tail() {
//        *link.borrow_mut() = Rc::clone(&b);
//    }
//    println!("b rc count after changing a = {}", Rc::strong_count(&b));
//    println!("a rc count after changing a = {}", Rc::strong_count(&a));
//    // The following line demonstrates a reference cyle and overflows the stack
//    // println!("a next item = {:?}", a.tail());
//}
