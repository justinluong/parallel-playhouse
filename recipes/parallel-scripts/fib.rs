use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::env;

fn fib(n: usize) -> BigInt {
    let mut a = Zero::zero();
    let mut b = One::one();
    let mut temp;

    for _ in 0..n {
        temp = &a + &b;
        a = b;
        b = temp
    }

    a
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: usize = args[1].parse().expect("Please provide a valid u64 number.");
    fib(n);
}