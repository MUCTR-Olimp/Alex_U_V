use std::io;

fn main(){
    let mut sum_of_digits = [0_usize;10000];
    for i in 0..10000 {
        let mut s = 0;
        let mut e = i;
        while e > 0 {
            s+= e%10;
            e/= 10;
        }
        sum_of_digits[i as usize] = s;
    }
    let mut count_array = [0; 37];
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let n = input.trim().parse::<i32>().unwrap();
    let b = 10_i32.pow((n/2) as u32);
    for i in 0..b{
        count_array[sum_of_digits[i as usize]]+=1;
    }
    let mut ans = 0;
    for i in count_array.iter(){
        ans+=(*i)*(*i);
    }
    println!("{}", ans);
}