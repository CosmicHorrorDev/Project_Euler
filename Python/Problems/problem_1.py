#!/usr/bin/env python3

from Utils.misc import sum_range

def solution():
    UPPER = 1000
    mult_sum = sum_of_multiples_of_3_and_5(UPPER)
    return f'The sum of multiples of 3 and 5 below {UPPER} is {mult_sum}'


def sum_of_multiples_of_3_and_5(UPPERBOUND):
    UPPERBOUND -= 1
    sum_mult = 5 * sum_range(UPPERBOUND // 5)
    sum_mult += 3 * sum_range(UPPERBOUND // 3)
    sum_mult -= (5 * 3) * sum_range(UPPERBOUND // (5 * 3))

    return sum_mult


