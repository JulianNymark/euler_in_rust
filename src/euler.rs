use std::{collections::HashMap, ops::Range};

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

    #[test]
    fn problem_6() {
        assert_eq!(super::problem_6(11), 2640)
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
    let mut prime_factors: Vec<Vec<i64>> = vec![];

    for num in range {
        let factorization = prime_factorize(num);
        prime_factors.push(factorization);
    }

    let mut tally_factors: HashMap<i64, i64> = HashMap::new();
    for factors in prime_factors {
        let mut current_tally_factors: HashMap<i64, i64> = HashMap::new();
        for factor in factors {
            let tally = current_tally_factors.get(&factor).unwrap_or(&0);
            current_tally_factors.insert(factor, tally + 1);
        }

        for current_tallied_factor in current_tally_factors {
            let existing = tally_factors.get(&current_tallied_factor.0).unwrap_or(&0);
            if *existing < current_tallied_factor.1 {
                tally_factors.insert(current_tallied_factor.0, current_tallied_factor.1);
            }
        }
    }

    let mut product_sum = 1;
    for tally_factor in &tally_factors {
        for _ in 0..*tally_factor.1 {
            product_sum *= tally_factor.0;
        }
    }
    return product_sum;
}

// sum square difference
// (a + b + c)^2 - ( a^2 + b^2 + c^2 )
pub fn problem_6(series_count: i64) -> i64 {
    let mut sum_squares_seq: i64 = 0;
    let mut sum_seq_squared: i64 = 0;
    for num in 1..series_count {
        sum_squares_seq += num.pow(2);
        sum_seq_squared += num;
    }
    sum_seq_squared = sum_seq_squared.pow(2);
    sum_seq_squared - sum_squares_seq
}
