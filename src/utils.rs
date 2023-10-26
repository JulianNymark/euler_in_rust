#[cfg(test)]
mod tests {
    #[test]
    fn is_prime() {
        let primes: Vec<i64> = vec![2, 3, 5];
        assert_eq!(super::is_prime(&primes, 3), true);
    }
    #[test]
    fn is_palindrome() {
        let mut primes: Vec<i64> = Vec::new();
        primes.push(2);
        assert_eq!(super::is_palindrome("racecar".to_string()), true);
        assert_eq!(super::is_palindrome("test".to_string()), false);
        assert_eq!(super::is_palindrome("1991".to_string()), true);
    }
    #[test]
    fn generate_primes() {
        assert_eq!(
            super::generate_primes(10),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
        assert_eq!(super::generate_primes(1), vec![2]);
        assert_eq!(super::generate_primes(0), vec![]);
    }
    #[test]
    fn integer_root() {
        assert_eq!(super::integer_root(25), 5);
        assert_eq!(super::integer_root(9), 3);
        assert_eq!(super::integer_root(10), 4);
        assert_eq!(super::integer_root(16), 4);
    }
    #[test]
    fn prime_factorize() {
        assert_eq!(super::prime_factorize(4), vec![2, 2]);
        assert_eq!(super::prime_factorize(7), vec![7]);
        assert_eq!(super::prime_factorize(8), vec![2, 2, 2]);
        assert_eq!(super::prime_factorize(10), vec![2, 5]);
    }
}

/// assumes that the root of the 'maybe_prime' is smaller than the largest
/// prime in the primes vector
pub fn is_prime(primes: &Vec<i64>, maybe_prime: i64) -> bool {
    let greater_than_root = integer_root(maybe_prime as i32) as i64;

    let primes_not_owner: &Vec<i64> = primes;
    for p in primes_not_owner {
        if *p > greater_than_root {
            break;
        }
        if maybe_prime % *p == 0 {
            return false;
        }
    }
    return true;
}

/// return a vector of primes that has 'length' elements
pub fn generate_primes(length: i32) -> Vec<i64> {
    if length == 0 {
        return vec![];
    }

    let mut primes: Vec<i64> = vec![2];

    let mut i: i64 = 3;
    while length > primes.len() as i32 {
        if is_prime(&mut primes, i) {
            primes.push(i);
        };
        i += 2;
    }

    return primes;
}

pub fn reverse(input: &String) -> String {
    let mut vector: Vec<char> = Vec::new();
    for c in input.chars() {
        vector.insert(0, c);
    }
    vector.into_iter().collect()
}

pub fn is_palindrome(input: String) -> bool {
    if input == reverse(&input) {
        return true;
    }
    false
}

/// the nearest integer root that's equal to or bigger than the real root of 'input'
pub fn integer_root(input: i32) -> i32 {
    let mut becomes_greater_than_root = 2;
    while becomes_greater_than_root * becomes_greater_than_root < input {
        becomes_greater_than_root += 1;
    }
    becomes_greater_than_root
}

pub fn prime_factorize(input: i64) -> Vec<i64> {
    let mut prime_factors = vec![];

    let integer_root = integer_root(input as i32);
    let primes = generate_primes(integer_root);

    if is_prime(&primes, input) {
        return vec![input];
    }

    for p in primes {
        if input % p == 0 {
            let mut exponent_counter = 1;
            let mut value = input;

            value = value / p;
            while value % p == 0 {
                value = value / p;
                exponent_counter += 1;
            }

            for _ in 0..(exponent_counter)  {
                prime_factors.push(p);
            }
        }
    }

    prime_factors
}
