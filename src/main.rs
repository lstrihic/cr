// https://doc.rust-lang.org/stable/rust-by-example/std/rc.html
// https://github.com/sunface/rust-by-practice/tree/master/solutions
// https://lise-henry.github.io/books/trpl2.pdf
// https://google.github.io/comprehensive-rust/control-flow-basics/blocks-and-scopes/scopes.html

// #[tokio::main]


use std::cell::{Cell, RefCell};
use std::rc::Rc;

fn say_hello(name: String) {
    println!("Hello {name}")
}

fn say_hello_ref(name: &String) {
    println!("Hello {name}")
}

#[derive(Debug, Default)]
struct Node {
    value: Cell<i32>,
    ref_value: RefCell<i32>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // rust ownership rules
    let s1: String = String::from("Hello!");
    let s2: String = s1; // s1 out of the scope it does not own anything:
    println!("s2: {s2}");
    // println!("s1: {s1}");


    let s1: String = String::from("Hello!");
    say_hello(s1); // the same happens here, the function is a new owner
    println!("s2: {s2}");
    // println!("s1: {s1}");

    // but this
    let s1: String = String::from("Hello!");
    say_hello_ref(&s1);
    println!("s2: {s2}");
    println!("s1: {s1}");

    // coping for some types are by default
    let x = 42;
    let y = x;
    println!("x: {x}"); // would not be accessible if not Copy
    println!("y: {y}");

    // smart pointer
    let value = Box::new(24);

    println!("{}", value); // box does implement Deref so we can directly access value


    let a = Rc::new(10);
    let b = Rc::clone(&a); // is cheap: it creates a pointer to the same allocation and increases the reference count.
    // Does not make a deep clone and can generally be ignored when looking for performance issues in code.

    println!("a: {:p}", a);
    println!("b: {:p}", b);

    // print_int(a);
    // print_int(b.into());


    let node = Node {
        value: Cell::new(13),
        ref_value: RefCell::new(43),
    };

    node.value.set(341);
    *node.ref_value.borrow_mut() = 34;


    println!("{:?}", node.value);


    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];



    println!("s: {s:?}");


    // migrations::run_migration().await.expect("error");

    Ok(())
}

fn print_int(x: &i32) {
    print!("{}\n", x);
}
