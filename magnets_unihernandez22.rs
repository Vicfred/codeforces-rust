use std::io::stdin;

fn main() {
    let mut n = String::new();
    stdin().read_line(&mut n).unwrap();
    let mut n: i64 = n.trim().parse().unwrap();

    let mut total = 1;
    let mut last = "";

    while n > 0 {
        let mut x = String::new();
        stdin().read_line(&mut x).unwrap();
        if last == x {
            total += 1;
        }
        last = &x;
        n -= 1;
    }

    println!("{}", total);
}
