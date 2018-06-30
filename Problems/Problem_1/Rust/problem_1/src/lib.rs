pub fn sum_multiples(upper: usize, multiples: &[usize]) -> usize {
    let mut sum = 0;

    for i in 1..upper {
        if multiples.iter().any(|&m| i % m == 0) {
            sum += i;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::sum_multiples;

    #[test]
    fn test_sum_multiples() {
        assert_eq!(sum_multiples(10, &[3, 5]), 23);
    }
}
