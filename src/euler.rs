use std::ops::Range;

use num::integer;

use crate::utils::{self, generate_primes, integer_root, prime_factorize};

#[cfg(test)]
mod tests {
    #[test]
    fn problem_1() {
        assert_eq!(super::problem_1(10), 23);
    }

    #[test]
    fn problem_2() {
        assert_eq!(super::problem_2(90), 44);
    }

    #[test]
    fn problem_3() {
        assert_eq!(super::problem_3(13195), 29);
    }

    #[test]
    fn problem_4() {
        assert_eq!(super::problem_4(2), 9009);
    }

    #[test]
    fn problem_5() {
        assert_eq!(super::problem_5(1..11), 2520);
    }
}

pub fn problem_1(upper: i32) -> i32 {
    let mut sum = 0;
    for i in 0..upper {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

///
/// NOTE: there is a functional way to do this too!
///
pub fn problem_2(upper: i32) -> i32 {
    let mut sum = 0;
    let mut previous = 1;
    let mut current = 2;
    while current < upper {
        if current % 2 == 0 {
            sum += current;
        }
        let next = previous + current;
        previous = current;
        current = next;
    }
    sum
}

pub fn problem_3(input: i64) -> i64 {

    let integer_root = integer_root(input as i32);
    let primes: Vec<i64> = generate_primes(integer_root);

    let mut max = 1;
    for p in primes {
        if input % p == 0 {
            max = p;
        }
    }

    max
}

pub fn problem_4(digits: i64) -> i64 {
    const BASE: i64 = 10;
    let lower = num::pow(BASE as i32, (digits - 1) as usize);
    let upper = num::pow(BASE as i32, digits as usize);

    let mut max: i64 = 0;
    for i in lower..upper {
        for j in lower..upper {
            let product: i64 = (i * j).into();
            if utils::is_palindrome(format!("{}", product)) && (product) as i64 > max {
                max = product;
            }
        }
    }

    max
}

// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
pub fn problem_5(range: Range<i64>) -> i64 {
    let integer_root = integer_root(range.end as i32);
    let primes = generate_primes(integer_root);

    let mut prime_factors: Vec<Vec<i64>> = vec![]; 

    for num in range {
        let factorization = prime_factorize(num);
        prime_factors.push(factorization);
    }

    for factors in prime_factors {
        // should probably use a better data structure here... like a hashmap... sad
    }

    return 42;
}