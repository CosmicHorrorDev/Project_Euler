use utils::misc::sum_range;


pub fn solution() -> String {
    let upper = 1000;
    let sum = sum_multiples_of_3_and_5(upper);
    format!("The sum of multiples of 3 and 5 below {} is {}", upper, sum)
}


fn sum_multiples_of_3_and_5(upper: usize) -> usize {
    let adj_upper = upper - 1;
    let mut sum = 5 * sum_range(adj_upper / 5);
    sum += 3 * sum_range(adj_upper / 3);
    sum -= 15 * sum_range(adj_upper / 15);

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

