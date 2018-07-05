#!/usr/bin/env python3


def sum_multiples_of_3_and_5(UPPERBOUND):
    UPPERBOUND -= 1
    sum_mult = 5 * sum_range(UPPERBOUND // 5)
    sum_mult += 3 * sum_range(UPPERBOUND // 3)
    sum_mult -= (5 * 3) * sum_range(UPPERBOUND // (5 * 3))

    return sum_mult


def sum_range(upper):
    sum_vals = ((upper + 1) // 2) * upper

    if not upper % 2:
        sum_vals += upper / 2

    return sum_vals


if __name__ == '__main__':
    sum_multiples_of_3_and_5(1000)

