pub fn solution() -> String {
    let upper = 4000000;
    let sum = sum_even_fibonacci(upper);
    format!("The sum of even Fibonacci numbers below {} is {}", upper, sum)
}


fn sum_even_fibonacci(upper: usize) -> usize {
    let mut sum = 2;

    let mut previous = 1;
    let mut current = 2;

    while previous + current < upper + 1 {
        current += previous;
        previous = current - previous;

        if current % 2 == 0 {
            sum += current;
        }
    }

    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_even_fibonacci() {
        assert_eq!(sum_even_fibonacci(10), 10);
    }
}
