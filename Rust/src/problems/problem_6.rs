use utils::misc::sum_range;


pub fn solution() -> String {
    let upper = 100;
    let difference = sum_square_difference(upper);
    format!("The difference between the squared sum and the sum squared of \
numbers 1 through {} is {}", upper, difference)
}


fn sum_square_difference(upper: usize) -> usize {
    let sum = sum_range(upper);
    let sum_square: usize = (1..=upper).into_iter().map(|x| x * x).sum();
    let square_sum = sum * sum;

    square_sum - sum_square
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_square_difference() {
        assert_eq!(sum_square_difference(10), 2640);
    }
}
