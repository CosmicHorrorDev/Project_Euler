#!/usr/bin/env python3
'''
Usage:
    python_problem_runner [--help | --run PROBLEMS | --bench PROBLEMS]

Arguments:
    PROBLEMS             The problems to run in a comma separated list e.g. 1,3

OPTIONS:
    -h --help            Shows this screen
    -r --run PROBLEMS    Runs the specified problem's programs
    -b --bench PROBLEMS  Benchmarks the specified problem's programs

EXAMPLES:
    python_problem_runner -r 1
        Runs the first problem's solution
    python_problem_runner -b 3,1,2
        Benchmarks problems 3, 1, then 2
'''
import sys
from datetime import datetime
from docopt import docopt

sys.path.append('../Problems/Problem_1/Python')
sys.path.append('../Problems/Problem_2/Python')
sys.path.append('../Problems/Problem_3/Python')

import problem_1
import problem_2
import problem_3


def main():
    HIGHEST_PROBLEM = 3
    arguments = docopt(__doc__)
    if arguments['--bench']:
        numbers = arguments['--bench']
    else:
        numbers = arguments['--run']

    parsed_numbers = []
    for n in numbers.split(','):
        # TODO: need a try catch here for parsing failure
        n = int(n)
        if n < 1 or n > 3:
            print((f'Error parsing arg: number should be from 1 to'
                   f' {HIGHEST_PROBLEM}, saw {n}'))
        parsed_numbers.append(n)

    if arguments['--bench']:
        bench(parsed_numbers)
    else:
        run(parsed_numbers)


def run(numbers):
    for number in numbers:
        if number == 1:
            result = problem_one()
        elif number == 2:
            result = problem_two()
        elif number == 3:
            result = problem_three()
        elif number == 4:
            result = problem_four()

        print('===================================================')
        print(f'[Benchmarking Problem {number}]')
        print(result)
    print('===================================================')


def bench(numbers):
    for number in numbers:
        times = []
        for i in range(5000):
            start = datetime.now()

            if number == 1:
                problem_one()
            elif number == 2:
                problem_two()
            elif number == 3:
                problem_three()
            elif number == 4:
                problem_four()

            times.append((datetime.now() - start).microseconds)

        mean, deviation = standard_deviation(times)
        print('===================================================')
        print(f'[Benchmarking Problem {number}]')
        print(f'mean ± σ [µs]: {mean:.3f} ± {deviation:.3f}')
    print('===================================================')


def standard_deviation(samples):
    mean = sum(samples) / len(samples)

    sum_diff_sqrd = 0.0
    for sample in samples:
        sum_diff_sqrd += (sample - mean) ** 2

    deviation = (sum_diff_sqrd / len(samples)) ** 0.5

    return mean, deviation


def problem_one():
    UPPER = 1000
    mult_sum = problem_1.sum_of_multiples_of_3_and_5(UPPER)
    return f'The sum of multiples of 3 and 5 below {UPPER} is {mult_sum}'


def problem_two():
    UPPER = 4000000
    fib_sum = problem_2.sum_even_fibonacci(UPPER)
    return f'The sum of even Fibonacci numbers below {UPPER} is {fib_sum}'


def problem_three():
    VAL = 600851475143
    factor = problem_3.largest_prime_factor(VAL)
    return f'The largest prime factor of {VAL} is {factor}'


if __name__ == '__main__':
    main()

