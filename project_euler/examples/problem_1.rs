/**
# Multiples of 3 and 5.
 Exercise 1If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
 The sum of these multiples is **23**.
 Find the sum of all the multiples of 3 or 5 below **1000**.

*/

fn main() {
    println!("Result: {}", multiples_sum_to(1000));
}

fn multiples_sum_to(to: u32) -> u32 {
    (0..to).fold(0, |acc, x| {
        if x % 3 == 0 || x % 5 == 0 {
            acc + x
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::multiples_sum_to;

    #[test]
    fn testing_multiples_below_10() {
        assert_eq!(23, multiples_sum_to(10));
    }
}
