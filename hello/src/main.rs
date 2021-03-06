// Example taken from Ch2 of Programming Rust 2/e
use std::env;
use std::str::FromStr;

fn main() {
  println!("Hello, rust!");

  let first = env::args().skip(1).next();
  match first {

    Some(v) => {
      let mut d = u64::from_str(&v).expect("error parsing arg");
      for m in env::args().skip(2) {
        d = gcd(d, u64::from_str(&m).expect("error parsing arg"));
      }
      println!("The GCD of {:?} is {}", env::args(), d);
    }

    None => {
      println!("Usage: gcd NUMBER, ...");
      std::process::exit(1);
    },
  }
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
  assert!(n != 0 && m != 0);
  while m != 0 {  
    if m < n {
      let t = m;
      m = n;
      n = t;
    }
    m = m % n;
  }  
  n
}

#[test]
fn test_gcd() {
  assert_eq!(gcd(14, 15), 1);
  assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
  	3 * 7 * 11 * 13 * 19), 3 * 11);
}
