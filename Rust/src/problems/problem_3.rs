pub fn solution() -> String {
    let num = 600851475143;
    let prime = largest_prime_factor(num);
    format!("The largest prime factor is {}", prime)
}


fn largest_prime_factor(num: u64) -> u64 {
    let mut number = num;
    let mut index = 3;

    // Factor out 2 seperately so I can index by 2 instead of 1
    while number % 2 == 0 {
        number /= 2;
    }

    loop {
        while number % index== 0 {
            number /= index;
        }
        // All values are factored out, index is the largest prime factor
        if number == 1 {
            break;
        }
        index += 2;
    }

    index
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_prime_factor() {
        assert_eq!(largest_prime_factor(13195), 29);
    }
}
