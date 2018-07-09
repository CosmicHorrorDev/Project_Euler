#!/usr/bin/env python3


<<<<<<< HEAD:Problems/Problem_2/Python/problem_2.py
=======
def solution():
    UPPER = 4000000
    fib_sum = sum_even_fibonacci(UPPER)
    return f'The sum of even Fibonacci numbers below {UPPER} is {fib_sum}'


>>>>>>> refactorProblemRunners:Python/Problems/problem_2.py
def sum_even_fibonacci(upperbound):
    result = 2

    previous = 1
    current = 2

    while previous + current < upperbound + 1:
        current, previous = current + previous, current

        if not current % 2:
            result += current

    return result

<<<<<<< HEAD:Problems/Problem_2/Python/problem_2.py

if __name__ == '__main__':
    UPPERBOUND = 4000000
    fib_sum = sum_even_fibonacci(4000000)
    print(f'Sum of even Fibonacci numbers below {UPPERBOUND} is {fib_sum}')

=======
>>>>>>> refactorProblemRunners:Python/Problems/problem_2.py
