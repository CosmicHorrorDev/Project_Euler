pub fn sum_multiples_of_3_and_5(upper: u64) -> u64 {
    let adj_upper = upper - 1;
    let mut sum = 5 * sum_range(adj_upper / 5);
    sum += 3 * sum_range(adj_upper / 3);
    sum -= 15 * sum_range(adj_upper / 15);

    sum
}


fn sum_range(upper: u64) -> u64 {
    let mut sum = ((upper + 1) / 2) * upper;

    if upper % 2 == 0 {
        sum += upper / 2;
    }

    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_multiples_of_3_and_5() {
        assert_eq!(sum_multiples_of_3_and_5(10), 23);
    }
}
