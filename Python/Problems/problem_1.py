#!/usr/bin/env python3

from Utils.misc import sum_range

def solution():
    UPPER = 1000
    mult_sum = sum_of_multiples_of_3_and_5(UPPER)
    return f'The sum of multiples of 3 and 5 below {UPPER} is {mult_sum}'


def sum_multiples_of_3_and_5(UPPERBOUND):
    UPPERBOUND -= 1
    sum_mult = 5 * sum_range(UPPERBOUND // 5)
    sum_mult += 3 * sum_range(UPPERBOUND // 3)
    sum_mult -= (5 * 3) * sum_range(UPPERBOUND // (5 * 3))

    return sum_mult


<<<<<<< HEAD:Problems/Problem_1/Python/problem_1.py
def sum_range(upper):
    sum_vals = ((upper + 1) // 2) * upper

    if not upper % 2:
        sum_vals += upper / 2

    return sum_vals


if __name__ == '__main__':
    sum_multiples_of_3_and_5(1000)

=======
>>>>>>> refactorProblemRunners:Python/Problems/problem_1.py
