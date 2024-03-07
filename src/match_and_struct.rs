#![allow(dead_code)]

use std::cmp::Ordering;
use std::i32;

#[derive(Debug)]
struct Pair(i32, i32);

fn main() {
    // let pair = Pair { 0: 12, 1: 23 };
    // let pair = Pair(2, 3);

    // let Pair(first, second) = pair;
    // println!("{:?}", pair);
    // println!("{} - {}", first, second);

    let num = 23;

    match num {
        1..=30 => println!("smaller than 30"),
        31 | 32 => println!("vlaue is 31 or 32"),
        _ => println!("gtrater than 32"),
    };

    let age = 18;
    let voting_age = 21;

    match age.cmp(&voting_age) {
        Ordering::Equal => println!("can vote"),
        Ordering::Greater => println!("can vote"),
        Ordering::Less => println!("can not vote"),
    };
}
