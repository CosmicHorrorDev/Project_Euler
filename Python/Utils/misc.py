def sum_range(upper):
    sum_vals = ((upper + 1) // 2) * upper

    if not upper % 2:
        sum_vals += upper / 2

    return sum_vals

