use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    print_list(&b);
}

enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

fn print_list(list: &List<i32>) {
    match list {
        Cons(value, next) => {
            println!("{}", value);
            print_list(next);
        }
        Nil => println!("Nil"),
    }
}
