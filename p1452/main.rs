use std::vec::Vec;
use std::collections::HashMap;
use std::io;
#[derive(Copy,Clone,Debug)]
struct Part {
  next: Option<u16>,
  prev: Option<u16>,
  ind: u16
}
#[derive(Debug)]
struct Element {
  value: u32,
  index: u16,
  parts: HashMap<u32,Part>,
}
fn main() {
  let stdin = io::stdin();
  let mut input = String::new();
  stdin.read_line(&mut input);
  let n = input.trim().parse::<usize>().unwrap();
  input.clear();
  stdin.read_line(&mut input);
  let mut s = input.trim().split(' ').enumerate().map(|i| Element{ value:i.1.parse::<u32>().unwrap(),index: i.0 as u16,parts: HashMap::new()}).collect::<Vec<_>>();
  s.sort_unstable_by(|a,b|
    if a.value < b.value {
      std::cmp::Ordering::Less
    } else if a.value > b.value {
      std::cmp::Ordering::Greater
    } else { std::cmp::Ordering::Equal}
  );
  let mut max = (s[1].value - s[0].value,0,0);//M{diff: s[1].value - s[0].value, count: 0, ind: 0};
  for i in 0..n {
    if i > 0 && s[i].value == s[i - (i > 0) as usize].value {continue;}
    for j in i+1..n {
      let t = s[j].value - s[i].value;
      match s[i].parts.get(&t) {
        Some(&p @ Part{next: Some(_),..}) => {
          if(t == 0) {
            let mut l = p.next.unwrap() as usize;
            while let Some(&Part{next: Some(k),..}) = s[l].parts.get(&t) {l = k as usize;}
            let &lp = s[l].parts.get(&t).unwrap();
            s[l].parts.insert(t, Part{next: Some(j as u16), ..lp});
            s[j].parts.insert(t, Part{next: None, ind: lp.ind+1, prev: Some(l as u16)});
            if max.1 <= lp.ind+1 {
              max = (t,lp.ind+1,j as u16);
            }
          }
        },
        Some(&p @ Part{next: None,..}) => {
          s[i].parts.insert(t, Part{next: Some(j as u16), ..p});
          s[j].parts.insert(t, Part{next: None, ind: p.ind+1, prev: Some(i as u16)});
          if max.1 <= p.ind+1 {
            max = (t,p.ind+1, j as u16);
          }
        }
        None => {
          s[i].parts.insert(t,Part{next: Some(j as u16), ind: 1, prev: None});
          s[j].parts.insert(t,Part{next: None, ind: 2, prev: Some(i as u16)});
          if max.1 <= 2 {
            max = (t,2,j as u16);
          }
        }
      }
    }
  }
  let mut count = max.2 as usize;
  println!("{}",max.1);
  for _ in 0..max.1 {
    //print!("{} ", l.index );
    let l = &s[count];
    print!("{} ",l.index+1);
    //println!("{:?}",l);
    match l.parts.get(&max.0) {
      None => break,
      Some(m) => count = match m.prev {
        Some(i) => i as usize,
        None => break,
      }
    };
  }
}