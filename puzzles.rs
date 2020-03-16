// https://codeforces.com/problemset/problem/337/A
use std::io;

fn main() {
    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .unwrap();

    let words: Vec<i64> =
        line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let n = words[0];

    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .unwrap();

    let mut f: Vec<i64> =
        line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    f.sort();

    println!("{:?}", f);

    let mut minima = 999;

    let mut iterator = f.iter();

    let mut first = f.next();
    let mut last = f.next();

    for _ in 0..n-1 {
        last = f.next();
    }

    println!("{}", minima);
}
