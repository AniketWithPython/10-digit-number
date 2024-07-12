use rayon::prelude::*;
use std::time::Instant;

fn countchar_iseq(a: char, s: &[char], i: usize) -> bool {
    let num = s.iter().filter(|c| **c == a).count();

    num == s[i].to_digit(10).unwrap().try_into().unwrap()
}

fn check(s: u128) {
    let check: Vec<char> = s.to_string().chars().collect();

    for i in 0..10 {
        if !countchar_iseq(char::from_digit(i, 10).unwrap(), &check, i as usize) {
            return;
        }
    }
    println!("{}", s);
}

fn main() {
    let now = Instant::now();

    (1_000_000_000..10_000_000_000)
        .into_par_iter()
        .for_each(check);

    println!("Time taken: {:?}", now.elapsed())
}

// Answer is 6210001000
