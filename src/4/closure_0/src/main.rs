use std::thread;
use std::time::Duration;

fn main() {
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    let n = example_closure(5); // error[E0308]: mismatched types
}

