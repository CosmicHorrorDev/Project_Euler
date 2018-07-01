extern crate time;
extern crate problem_1;
extern crate problem_2;
extern crate problem_3;

use std::env::args;
use std::process::exit;
use time::precise_time_ns;


fn main() {
    let problem_functions = vec![
        problem_one as fn() -> String,
        problem_two as fn() -> String,
        problem_three as fn() -> String,
    ];

    let mut usage = String::new();
    usage += "USAGE:\n";
    usage += "\trust_problem_runner [--help | --run PROBLEMS | --bench PROBLEMS]\n\n";
    usage += "OPTIONS:\n";
    usage += "\t-h --help            Shows this screen\n";
    usage += "\t-r --run PROBLEMS    Runs the specified problem's programs given in a comma separated list\n";
    usage += "\t-b --bench PROBLEMS  Benchmarks the specified problem's programs given in a comma separated list\n\n";
    usage += "EXAMPLES:\n";
    usage += "\trust_problem_runner -r 1\n";
    usage += "\t\tRuns the first problem's solution\n";
    usage += "\trust_problem_runner -b 3,1,2\n";
    usage += "\t\tBenchmarks problems 3, 1, then 2\n";

    let mut parsed = Vec::new();
    for arg in args().skip(1) {
        parsed.push(arg);
    }

    // The below code is all to verify correct usage of cli args
    // 1 arg if -h flag, 2 for -r or -b to give problem numbers
    if parsed.len() < 1 || parsed.len() > 2 {
        panic!(usage);
    }

    let bench_probs = parsed[0] == "-b" || parsed[0] == "--bench";
    let run_probs = parsed[0] == "-r" || parsed[0] == "--run";
    let help = parsed[0] == "-h" || parsed[0] == "--help";

    // Incorrect usage if no flags, or run and bench flags
    if !(bench_probs || run_probs || help) || (bench_probs && run_probs) {
        panic!(usage)
    }

    if help {
        println!("{}", usage);
        exit(0);
    }

    // Parse problem numbers to run
    let mut nums = Vec::new();
    for num in parsed[1].split(",") {
        nums.push(match num.parse() {
            Err(_) => {
                panic!(format!("Error parsing arg, expected int, saw {}",
                               num));
            }
            Ok(n) => {
                assert!(n > 0 && n <= problem_functions.len());
                n
            }
        });
    }

    if bench_probs {
        bench(problem_functions, &nums);
    } else {
        run(problem_functions, &nums);
    }
}


fn run<F>(problems: Vec<F>, numbers: &[usize])
where F: Fn() -> String
{
    for number in numbers.iter() {
        println!("===================================================");
        println!("[Running Problem {}]", *number);
        println!("{}", problems[*number - 1]());
    }
    println!("===================================================");
}


fn bench<F>(problems: Vec<F>, numbers: &[usize])
where F: Fn() -> String
{
    for number in numbers.iter() {
        let mut times = Vec::new();
        // Eventually dig in to automatically determining sample size but for
        // now this will do
        let mut index = 10000;
        while index > 0 {
            let start = precise_time_ns();
            problems[*number - 1]();
            times.push(precise_time_ns() - start);

            index -= 1;
        }

        let (mean, deviation) = standard_deviation(&times);
        println!("===================================================");
        println!("[Benchmarking Problem {}]", *number);
        println!("mean ± σ [µs]: {:.3} ± {:.3}",
                 mean / 1000.0, deviation / 1000.0);
    }
    println!("===================================================");
}


fn standard_deviation(samples: &Vec<u64>) -> (f64, f64) {
    let sum: u64 = samples.iter().sum();
    let mean = (sum as f64) / (samples.len() as f64);
    let mut sum_diff_sqrd = 0.0;
    for sample in samples {
        sum_diff_sqrd += ((*sample as f64) - mean).powi(2);
    }
    let deviation = (sum_diff_sqrd / (samples.len() as f64)).sqrt();

    (mean, deviation)
}


fn problem_one() -> String {
    let upper = 1000;
    let sum = problem_1::sum_multiples_of_3_and_5(upper);
    format!("The sum of multiples of 3 and 5 below {} is {}", upper, sum)
}


fn problem_two() -> String {
    let upper = 4000000;
    let sum = problem_2::sum_even_fibonacci(upper);
    format!("The sum of even Fibonacci numbers below {} is {}", upper, sum)
}


fn problem_three() -> String {
    let num = 600851475143;
    let prime = problem_3::largest_prime_factor(num);
    format!("The largest prime factor is {}", prime)
}

