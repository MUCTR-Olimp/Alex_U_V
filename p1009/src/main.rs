use std::io;
fn calc_nums(ind : i32, prev_is_zero: bool,k: i32) -> i32 {
    if ind == 0 {
        if prev_is_zero {
            k-1
        } else {
            k
        }
    } else {
        if prev_is_zero {
            (k-1)*calc_nums(ind-1, false,k)
        } else {
            calc_nums(ind-1, true,k)+(k-1)*calc_nums(ind-1,false,k)
        }
    }
}
fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input);
    let n = input.trim().parse::<i32>().unwrap();
    input.clear();
    stdin.read_line(&mut input);
    let k = input.trim().parse::<i32>().unwrap();
    let ans = calc_nums(n-1, true,k);
    println!("{}", ans);
}
