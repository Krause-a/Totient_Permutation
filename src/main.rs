#![allow(unused)]

use std::time::{Instant, Duration};

//Starting at 15:00

/* ### Totient Perutation - Problem 70
Euler's totient function, phi(n) [sometimes called the phi function], is used to determine the number of positive numbers less than or equal to n which are relatively prime to n. 
For example, as 1, 2, 4, 5, 7, and 8, are all less than nine and relatively prime to nine, phi(9)=6.    The number 1 is considered to be relatively prime to every positive number, so phi(1)=1. 
Interestingly, phi(87109)=79180, and it can be seen that 87109 is a permutation of 79180.
Find the value of n, 1 lt n lt 10^7, for which phi(n) is a permutation of n and the ratio n over phi(n) produces a minimum.
A minimum is the least of all n that does produce a permutation.
*/

fn main() {
    let args : Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected exactly one argument.");
        return;
    }
    let Ok(value) = args[1].parse::<usize>() else {
        eprintln!("Could not parse argument as usize.");
        return;
    };

    let mut tracked_value = usize::MAX;
    let mut tracked_totient = usize::MAX;
    let mut smallest_ratio = f64::MAX;

    let mut times : Vec<Duration> = Vec::new();

    for i in 2..=value {
        let inst = Instant::now();
        let tot = totient(i);
        times.push(inst.elapsed());
        //println!("{}", i);
        //println!("Totient Time: {:?}", inst.elapsed());
        if check_permutation(i, tot) {
            let ratio = i as f64 / tot as f64;
            if ratio < smallest_ratio {
                smallest_ratio = ratio;
                println!("phi({}) = {} -> {}", i, tot, ratio);
            }
        }
        //println!("Permutation Time: {:?}", inst.elapsed());
    }
}

fn check_permutation(a:usize, b:usize) -> bool {
    count_digits(a) == count_digits(b)
}

fn count_digits(n:usize) -> [usize; 10] {
    let mut counts = [0; 10];
    let mut num = n;

    while num > 0 {
        counts[(num % 10)] += 1;
        num /= 10;
    }

    counts
}

//All positive numbers less than n that are co-prime to n
fn totient(n:usize) -> usize {
    let mut count = 0;
    for i in 1..n {
        if check_co_prime(i, n) {
            count += 1;
        }
    }
    count
}

fn check_co_prime(a:usize, b:usize) -> bool {
    gcd(a, b) == 1
}

fn gcd(a:usize, b:usize) -> usize {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
