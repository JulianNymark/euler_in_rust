mod euler;
mod utils;

fn main() {
    println!("problem 1: {}", euler::problem_1(1_000));
    println!("problem 2: {}", euler::problem_2(4_000_000));
    println!("problem 3: {}", euler::problem_3(600851475143));
    println!("problem 4: {}", euler::problem_4(3));
    println!("problem 5: {}", euler::problem_5(1..21));
}
