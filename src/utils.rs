#[cfg(test)]
mod tests {
    #[test]
    fn add_if_prime() {
        let mut primes: Vec<i64> = Vec::new();
        primes.push(2);
        super::add_if_prime(&mut primes, 3);
        assert_eq!(primes, [2,3]);
    }
    #[test]
    fn is_palindrome() {
        let mut primes: Vec<i64> = Vec::new();
        primes.push(2);
        assert_eq!(super::is_palindrome("racecar".to_string()), true);
        assert_eq!(super::is_palindrome("test".to_string()), false);
        assert_eq!(super::is_palindrome("1991".to_string()), true);
    }
}

pub fn add_if_prime(primes: &mut Vec<i64>, maybe_prime: i64) {
    let mut greater_than_root = 2;
    while greater_than_root * greater_than_root < maybe_prime {
        greater_than_root += 1;
    }

    let mut is_prime = false;

    let primes_not_owner: &Vec<i64> = primes;
    for p in primes_not_owner {
        if *p >= greater_than_root {
            is_prime = true;
            break;
        }
        if maybe_prime % *p == 0 {
            break;
        }
    }
    if is_prime {
        primes.push(maybe_prime);
    }
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
