use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let n_elements: usize = lines.next().unwrap().parse().unwrap();
    dbg!(n_elements);
    
    lines
    .map(|x| x.parse().unwrap())
    .map(|x: u32| euler_1(x))
    .for_each(|x| println!("{}", x))
}

fn euler_1(n: u32) -> u32 {
    let mut sum = 0;
    for i in 0..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}
