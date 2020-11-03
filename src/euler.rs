#[cfg(test)]
mod tests {
    #[test]
    fn problem_1() {
        let answer = super::problem_1(10);
        assert_eq!(answer, 23);
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
