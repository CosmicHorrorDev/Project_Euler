#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate time;

pub mod problems;
pub mod utils;

use problems::*;
use std::process::exit;
use time::precise_time_ns;
use docopt::Docopt;

const USAGE: &'static str = "
Usage:
    rust_problem_runner (--help)
    rust_problem_runner [--run PROBLEMS | --bench PROBLEMS]

Options:
    -h --help            Shows this screen
    -r --run PROBLEMS    Runs the specified problem's solution
    -b --bench PROBLEMS  Benchmarks the specified problem's solution

Arguments:
    PROBLEMS  Problems can be specified with 3 forms where # is a number
                  all: all is special and specifies all solutions
                  #,#: will run the first number, then the second and can be
                       chained like #,#,#,#
                  #:#: will run all the soltuions from the first number to the
                       second number, can be chained with commas like #:#,#:#

Examples:
    rust_problem_runner -r 1
        Runs the first problem's solution
    rust_problem_runner -b 3,1,2
        Benchmarks problems 3, 1, then 2
";
#[derive(Debug, Deserialize)]
struct Args {
    flag_run: String,
    flag_bench: String,
}


fn main() {
    let problem_functions = vec![
        //problem_one as fn() -> String,
        problem_1::solution as fn() -> String,
        problem_2::solution as fn() -> String,
        problem_3::solution as fn() -> String,
        problem_4::solution as fn() -> String,
        problem_5::solution as fn() -> String,
        problem_6::solution as fn() -> String,
        problem_7::solution as fn() -> String,
        problem_8::solution as fn() -> String,
        problem_9::solution as fn() -> String,
    ];

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.deserialize())
                            .unwrap_or_else(|e| e.exit());
    let problems = if args.flag_bench != "" {
        &args.flag_bench
    } else {
        &args.flag_run
    };

    let numbers = parse_problems(problems, problem_functions.len());

    if args.flag_bench != "" {
        bench(problem_functions, &numbers);
    } else {
        run(problem_functions, &numbers);
    }
}


fn parse_problems(raw: &str, high_bound: usize) -> Vec<usize> {
    let mut parsed = Vec::new();
    if raw == "all" {
        parsed = (1..=high_bound).collect();
    } else {
        for section in raw.split(',') {
            if section.contains(':') {
                let limits: Vec<&str> = section.split(':').collect();
                if limits.len() != 2 {
                    println!("Error: ranges cannot be chained and must be in \
                             the form of #:# like 1:3");
                    exit(1);
                }

                let start = parse_with_high_limit(limits[0], high_bound);
                let end = parse_with_high_limit(limits[1], high_bound);

                // look into matching for Ordering for here
                if start < end {
                    parsed.append(&mut (start..=end).collect());
                } else {
                    parsed.append(&mut (end..=start).rev().collect());
                }
            } else {
                parsed.push(parse_with_high_limit(section, high_bound));
            }
        }
    }
    parsed
}


fn parse_with_high_limit(raw: &str, high_bound: usize) -> usize {
    match raw.parse() {
        Err(e) => {
            println!("Error Parsing Number: {}", e);
            exit(1);
        },
        Ok(num) => {
            if num < 1 || num > high_bound {
                println!("Error Parsing Number: {} should be \
                         within 1 and {}", raw,
                         high_bound);
                exit(1);
            }

            num
        }
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
    let max_rounds = 10000;
    let min_rounds = 10;
    for number in numbers.iter() {
        println!("===================================================");
        println!("[Benchmarking Problem {}]", *number);

        // Run through min_rounds to see how many more rounds should be done
        let mut times = time_solution(&problems[*number - 1], min_rounds);

        // Floored division shouldn't be a problem here
        // Get average to see how many more rounds can be done in 1 minute
        let sum: u64 = times.iter().sum();
        let mean = sum as usize / times.len();
        let mut rounds = 60000000000 / mean;
        if rounds < 10 {
            rounds = 0;
        } else if rounds > max_rounds - min_rounds {
            rounds = max_rounds - min_rounds;
        }

        times.append(&mut time_solution(&problems[*number - 1], rounds));

        let (mean, deviation) = standard_deviation(&times);
        println!("mean ± σ [µs]: {:.2} ± {:.2}",
                 mean / 1000.0, deviation / 1000.0);
    }
    println!("===================================================");
}


fn time_solution<F>(problem: F, rounds: usize) -> Vec<u64>
where F: Fn() -> String
{
    let mut index = 0;
    let mut times = Vec::with_capacity(rounds);
    while index < rounds {
        let start = precise_time_ns();
        problem();
        times.push(precise_time_ns() - start);

        index += 1;
    }

    times
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

