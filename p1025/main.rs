use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input);
    let n = input.trim().parse::<i32>().unwrap();
    let mut groups:std::vec::Vec<i32> = std::vec::Vec::new();
    input.clear();
    stdin.read_line(&mut input);
    let mut groups = input.trim().split(' ').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    groups.sort();
    let mut ans = 0;
    for i in 0..n/2+1 {
        ans+=groups[i as usize]/2+1;
    }
    println!("{}", ans);
}