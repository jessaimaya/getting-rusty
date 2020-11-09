/**
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

Run: cargo run --example problem_5
Test: cargo test --example problem_5

*/

fn main() {
    println!("Result: {}", prime_factor(600851475143));
}

fn smallest_div() -> u32 {
    let mut found = false;
    let mut count = 1u32;

    while !found {
        let mut f= true;
        for i in 1..=20 {
            if count & 1 != 0 || count % i != 0 {
                f = false;
                count += 1;
                break;
            }
        }
        if f == true {
            found = true;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::prime_factor;

    #[test]
    fn test_prime_factor() {
        assert_eq!(29, prime_factor(13195));
    }
}
