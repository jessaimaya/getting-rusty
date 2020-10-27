/**
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.

Run: cargo run --example problem_4
Test: cargo test --example problem_4

*/

fn main() {
    let mut result = 0;
    for i in (100..1000).rev() {
        for j in (100..1000).rev() {
            let mult = i*j;
            if string_palindrome(&mult.to_string()) == true && mult > result {
                result = mult;
            }
        }
    }
    println!("r: {}", result);
}

fn string_palindrome(n: &str) -> bool {
    n.chars().rev().collect::<String>() == n
}

#[cfg(test)]
mod tests {
    use crate::string_palindrome;

    #[test]
    fn test_string_palindrome() {
      assert!(string_palindrome(&9009.to_string()));
    }
}
