/**
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?

Run: cargo run --example problem_3
Test: cargo test --example problem_3

*/

fn main() {
    println!("Result: {}", prime_factor(600851475143));
}

fn prime_factor(mut n: usize) -> usize {
    let mut div = 2;
    while n > 1 {
        if n % div == 0 {
            n = n /div;
        } else {
            div += 1;
        }
    }
    div
}

#[cfg(test)]
mod tests {
    use crate::prime_factor;

    #[test]
    fn test_prime_factor() {
        assert_eq!(29, prime_factor(13195));
    }
}
