pub fn sum_range(upper: usize) -> usize {
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
    fn it_works() {
        for i in 1..100 {
            let known_sum = (0..=i).into_iter().sum();
            assert_eq!(sum_range(i), known_sum);
        }
    }
}

