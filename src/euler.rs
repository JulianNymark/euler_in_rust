#[cfg(test)]
mod tests {
    #[test]
    fn problem_1() {
        let answer = super::problem_1(10);
        assert_eq!(answer, 23);
    }

    #[test]
    fn problem_2() {
        let answer = super::problem_2(90);
        assert_eq!(answer, 44);
    }
}

pub fn problem_1(upper: i32) -> i32 {
    let mut sum = 0;
    for i in 0..upper {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    return sum;
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
    return sum;
}