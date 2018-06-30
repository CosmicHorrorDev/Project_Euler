extern crate time;
extern crate problem_1;

use time::precise_time_ns;

// TODO:
// * Parse command line args
// * Do better benchmarking calculations
fn main() {
    let problem_functions = vec![run_problem_one];
    let mut problem_numbers = Vec::new();
    for i in 0..problem_functions.len() {
        problem_numbers.push(i);
    }

    bench(problem_functions, &problem_numbers);
}


fn run<F>(problems: Vec<F>, numbers: &[usize])
where F: Fn() -> String
{
    for number in numbers.iter() {
        println!("{}", problems[*number]());
    }
}


fn bench<F>(problems: Vec<F>, numbers: &[usize])
where F: Fn() -> String
{
    let mut times = Vec::new();
    for number in numbers.iter() {
        let mut index = 10;
        while index > 0 {
            let start = precise_time_ns();
            problems[*number]();
            times.push(precise_time_ns() - start);

            index -= 1;
        }

        let times_sum: u64 = times.iter().sum();
        println!("Average time: {}", (times_sum as f32) / (times.len() as f32));
    }
}


fn run_problem_one() -> String {
    let upper = 1000;
    let multiples = &[3, 5];
    let sum = problem_1::sum_multiples(upper, multiples);
    format!("The sum of multiples of {:?} below {} is {}",
            multiples, upper, sum)
}

