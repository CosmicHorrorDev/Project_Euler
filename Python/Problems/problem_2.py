#!/usr/bin/env python3


def solution():
    UPPER = 4000000
    fib_sum = sum_even_fibonacci(UPPER)
    return f'The sum of even Fibonacci numbers below {UPPER} is {fib_sum}'


def sum_even_fibonacci(upperbound):
    result = 2

    previous = 1
    current = 2

    while previous + current < upperbound + 1:
        current, previous = current + previous, current

        if not current % 2:
            result += current

    return result


if __name__ == '__main__':
    print(solution())

