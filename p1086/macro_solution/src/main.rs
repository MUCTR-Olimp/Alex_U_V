use macro_solution::generate_primes;
use std::io;
fn main() {
    let primes = generate_primes!(170000);
    //println!("{:?}",primes);
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
