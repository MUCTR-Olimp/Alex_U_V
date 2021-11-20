use std::io;
fn main() {
    let border = 170000;
    let mut list: std::vec::Vec<i32> = (2..=border+1).map(|_| 1).collect();
    let mut index: usize = 2;
    while index < border/2{
        if list[index - 2] == 1 {
            for i in ((2*index-2)..border).step_by(index) {
                list[i] = 0;
            }
        }
        index += 1;
    }
    let mut primes = std::vec::Vec::new();
    for (i,x) in list.iter().enumerate() {
        if *x==1 {
            primes.push(i+2);
        }
    }
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input);
    let k = input.trim().parse::<i32>().unwrap();
    let mut n = 0;
    for _ in 0..k {
        input.clear();
        stdin.read_line(&mut input);
        n = input.trim().parse::<usize>().unwrap();
        n-=1;
        println!("{}",primes[n]);
    }
}