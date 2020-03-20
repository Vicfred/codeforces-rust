// https://codeforces.com/problemset/problem/61/A
// implementation, simulation
use std::io;

fn main() {
    let mut a = String::new();

    io::stdin()
        .read_line(&mut a)
        .unwrap();

    let a = a.trim();

    let a: Vec<char> = a.chars().collect();

    let mut b = String::new();

    io::stdin()
        .read_line(&mut b)
        .unwrap();

    let b = b.trim();

    let b: Vec<char> = b.chars().collect();

    let mut answer = String::from("");

    for i in 0..a.len() {
        if a[i] == b[i] {
            answer.push_str("0");
        } else {
            answer.push_str("1");
        }
    }

    println!("{}", answer);
}

