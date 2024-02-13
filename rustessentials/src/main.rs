mod arrays_ex;
mod vector_ex;
mod fibonacci;

use arrays_ex::arrays;
use vector_ex::vectors;
use fibonacci::fib_num as fib;

fn main() {
    println!("Hello, world!");
    arrays();
    vectors();
    fib();
}
