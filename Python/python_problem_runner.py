#!/usr/bin/env python3
'''
Usage:
    python_problem_runner (--help)
    python_problem_runner [--run PROBLEMS | --bench PROBLEMS]

Arguments:
    PROBLEMS  Problems can be specified with 3 forms where # is a number
                  all: all is special and specifies all solutions
                  #,#: will run the first number, then the second and can be
                       chained like #,#,#,#
                  #:#: will run all the soltuions from the first number to the
                       second number, can be chained with commas like #:#,#:#

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

sys.path.append('../Problems')

from Problems import problem_1, problem_2, problem_3, problem_4


def main():
    HIGHEST_PROBLEM = 4
    arguments = docopt(__doc__)
    if arguments['--bench']:
        numbers = arguments['--bench']
    else:
        numbers = arguments['--run']

    numbers = parse_problems(numbers, HIGHEST_PROBLEM)

    if arguments['--bench']:
        bench(numbers)
    else:
        run(numbers)


def parse_problems(raw, high_bound):
    parsed = []
    if raw == 'all':
        parsed = [i for i in range(1, high_bound + 1)]
    else:
        for section in raw.split(','):
            if ':' in section:
                limits = section.split(':')
                if len(limits) != 2:
                    print('Error: ranges cannot be chained and must be in the'
                          ' form of #:# like 1:3')
                    sys.exit(1)

                start = parse_with_high_limit(limits[0], high_bound)
                end = parse_with_high_limit(limits[1], high_bound)

                if start < end:
                    parsed += [i for i in range(start, end + 1)]
                else:
                    parsed += [i for i in range(start, end - 1, -1)]
            else:
                parsed.append(parse_with_high_limit(section, high_bound))
    return parsed


def parse_with_high_limit(raw, high_bound):
    try:
        parsed =  int(raw)
        if parsed > 0 and parsed <= high_bound:
            return parsed
        else:
            print(f'Error Parsing Number: {parsed} should be within 1 and'
                  f' {high_bound}')
            sys.exit(1)
    except ValueError:
        print(f'Error parsing number from: {raw}')


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
        print(f'[Running Problem {number}]')
        print(result)
    print('===================================================')


def bench(numbers):
    for number in numbers:
        times = []
        print('===================================================')
        print(f'[Benchmarking Problem {number}]')
        for i in range(10000):
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
        print(f'mean ± σ [µs]: {mean:.2f} ± {deviation:.2f}')
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


def problem_four():
    UPPER = 1000
    num1, num2 = problem_4.largest_palindrome_product(UPPER)
    product = num1 * num2
    return f'The largest palindrome product is {product} = {num1} * {num2}'


if __name__ == '__main__':
    main()

