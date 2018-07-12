pub fn sum_range(upper: usize) -> usize {
    let mut sum = ((upper + 1) / 2) * upper;

    if upper % 2 == 0 {
        sum += upper / 2;
    }

    sum
}
