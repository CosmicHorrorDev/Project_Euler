extern crate time;
extern crate problem_1;
extern crate problem_2;
extern crate problem_3;
extern crate problem_4;

use std::env::args;
use std::process::exit;
use time::precise_time_ns;


fn main() {
    let problem_functions = vec![
        problem_one as fn() -> String,
        problem_two as fn() -> String,
        problem_three as fn() -> String,
        problem_four as fn() -> String,
    ];

    let usage = "USAGE:
\trust_problem_runner [--help | --run PROBLEMS | --bench PROBLEMS]\n
OPTIONS:
\t-h --help            Shows this screen
\t-r --run PROBLEMS    Runs the specified problem's programs given in a comma separated list
\t-b --bench PROBLEMS  Benchmarks the specified problem's programs given in a comma separated list\n
EXAMPLES:
\trust_problem_runner -r 1
\t\tRuns the first problem's solution
\trust_problem_runner -b 3,1,2
\t\tBenchmarks problems 3, 1, then 2\n";

    let mut parsed = Vec::new();
    for arg in args().skip(1) {
        parsed.push(arg);
    }

    // The below code is all to verify correct usage of cli args
    // 1 arg if -h flag, 2 for -r or -b to give problem numbers
    if parsed.len() < 1 || parsed.len() > 2 {
        println!("{}", usage);
        exit(1);
    }

    let bench_probs = parsed[0] == "-b" || parsed[0] == "--bench";
    let run_probs = parsed[0] == "-r" || parsed[0] == "--run";
    let help = parsed[0] == "-h" || parsed[0] == "--help";

    // Incorrect usage if no flags, or run and bench flags
    if !(bench_probs || run_probs || help) || (bench_probs && run_probs) {
        println!("{}", usage);
        exit(1);
    }

    if help {
        println!("{}", usage);
        exit(0);
    }

    if (bench_probs || run_probs) && parsed.len() != 2 {
        println!("Expected 1 argument after --bench or --run, saw {}",
                 parsed.len() - 1);
        exit(1);
    }

    // move this to a function
    // Parse problem numbers to run
    let mut nums = Vec::new();
    if parsed[1] == "all" {
        nums = (1..=problem_functions.len()).collect();
    } else {
        for section in parsed[1].split(',') {
            if section.contains(':') {
                let elements: Vec<&str> = section.split(':').collect();
                assert_eq!(elements.len(), 2);

                let start = elements[0].parse().expect("Error: expected int");
                let end = elements[1].parse().expect("Error: expected int");
                nums.append(&mut (start..=end).collect());
            } else {
                nums.push(section.parse().expect("Error: expected int"));
            }
        }
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
        println!("===================================================");
        println!("[Benchmarking Problem {}]", *number);

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
        println!("mean ± σ [µs]: {:.2} ± {:.2}",
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


fn problem_four() -> String {
    let upper = 1000;
    let (num1, num2) = problem_4::largest_palindrome_product(upper);
    let product = num1 * num2;
    format!("The largest palindrome product is {} = {} * {}",
            product, num1, num2)
}

