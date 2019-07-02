use std::thread;
use std::time::Duration;

fn  add_one_v1   (x: u32) -> u32 { x + 1 }

fn main() {
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;
    println!("{}", add_one_v1(100));
    println!("{}", add_one_v2(100));
    println!("{}", add_one_v3(100));
    println!("{}", add_one_v4(100));
}

