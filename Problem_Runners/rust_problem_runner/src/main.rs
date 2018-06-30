extern crate time;
extern crate problem_1;

use time::precise_time_ns;


fn main() {
    run_problem_one();
}

fn run_problem_one() {
    let start = precise_time_ns();
    let upper = 1000;
    let multiples = &[3, 5];
    let sum = problem_1::sum_multiples(upper, multiples);
    println!("The sum of multiples of {:?} below {} is {}",
             multiples, upper, sum);
    println!("Execution time [ns]: {}", precise_time_ns() - start);
}

