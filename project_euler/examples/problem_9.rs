/**
A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

a2 + b2 = c2
For example, 32 + 42 = 9 + 16 = 25 = 52.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.

Run: cargo run --example problem_9
Test: cargo test --example problem_9

*/

fn main() {
  let triplet = {
      let mut r = 0;
      'a: for a in 1..1000 {
          for b in 1..1000 {
              let sum = (a*a) + (b*b);
              let c = 1000 - a - b;
              if sum == c * c {
                  r = a * b * c;
                  break 'a;
              }
          }
      }
      r
  };
  println!("{:?}", triplet);
    
}
