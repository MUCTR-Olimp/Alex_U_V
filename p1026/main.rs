use std::io;
use std::vec::Vec;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input);
    let n = input.trim().parse::<i32>().unwrap();
    input.clear();
    let mut db: Vec<i32> = Vec::new();
    for i in 0..n {
        stdin.read_line(&mut input);
        db.push( input.trim().parse::<i32>().unwrap());
        input.clear();
    }
    stdin.read_line(&mut input);
    input.clear();
    stdin.read_line(&mut input);
    let k = input.trim().parse::<i32>().unwrap();
    input.clear();
    let mut qurries: Vec<usize> = Vec::new();
    for i in 0..k {
        stdin.read_line(&mut input);
        qurries.push(input.trim().parse::<usize>().unwrap());
        input.clear();
    }
    db.sort();
    for i in qurries {
        println!("{}",db[i-1]);
    }
}