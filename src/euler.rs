use crate::utils;

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
        utils::add_if_prime(&mut primes, i);
    }

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
