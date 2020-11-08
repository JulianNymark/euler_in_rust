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
    let mut greater_than_root = 2;
    while greater_than_root * greater_than_root < input {
        greater_than_root += 1;
    }

    let mut primes: Vec<i64> = Vec::new();
    primes.push(2);

    for i in 3..greater_than_root {
        add_if_prime(&mut primes, i);
    }

    let mut max = 1;
    for p in primes {
        if input % p == 0 {
            max = p;
        }
    }

    max
}

fn add_if_prime(primes: &mut Vec<i64>, maybe_prime: i64) {
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
