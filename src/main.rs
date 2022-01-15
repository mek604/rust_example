use std::io::Write;
use std::str::FromStr;

use lib::*;

// src/main.rs root of binary crate

fn main() {

    let numbers = read_u64();
    let d = math::gcd(&numbers);

    println!("The gcd of {:?} is {}", numbers, d);
}


// #[test] attribute, marks fn as a test fn - skipped in normal compilations
// include only when run cargo test
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19, 3 * 11));
}

fn read_u64() -> Vec<u64> {
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
            .expect("error parsing argument")); 
        // if panics, error msg begins with "error parsing argument"
        // if not, return contained value
    }

    // one-liner
    // std::env::args().skip(1).for_each(|n| numbers.push(u64::from_str(&n)
    //     .expect("error parsing argument")));

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    numbers    
}
