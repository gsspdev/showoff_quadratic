use quadratic_residues::*;
use std::ffi;
use std::iter::FromIterator;

fn main() {
    let mut primes = Vec::with_capacity(100);
    let mut num = 2;
    while primes.len() < 100 {
        if is_prime(num) {
            primes.push(num);
        }
        num += 1;
    }
    println!("{:?}", primes);
}

    for n in prime.len() {
        quadratic_residues_all(n)
        big_quad_res.push(n);
    }

    let quad_seven = quadratic_residues_all(5);
    let all_quad_res_twenty_three = quadratic_residues_all(23);
    println!("quadratic residues of seven are {:?}", quad_seven);
    println!(
        "all quadratic residues of twenty three {:?}",
        all_quad_res_twenty_three
    );

pub fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}
