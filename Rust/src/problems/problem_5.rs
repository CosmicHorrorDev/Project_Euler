pub fn solution() -> String {
    let upper = 20;
    let multiple = smallest_multiple(upper);
    format!("The smallest psitive number divisible by all the numbers from \
1 to {} is {}", upper, multiple)
}


fn smallest_multiple(upper: usize) -> usize {
    let primes = prime_finder(upper);
    let mut prime_products = Vec::new();
    for prime in primes.iter() {
        let mut prime_product = *prime;
        while prime_product * *prime < upper {
            prime_product *= *prime;
        }
        prime_products.push(prime_product);
    }

    prime_products.iter().product()
}


fn prime_finder(upper: usize) -> Vec<usize> {
    let mut primes = vec![2];
    let mut index = 3;

    while upper > index {
        let mut is_prime = true;
        for prime in primes.iter() {
            if index % prime == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(index);
        }

        index += 1;
    }

    primes
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_multiple() {
        assert_eq!(smallest_multiple(10), 2520);
    }

    #[test]
    fn test_prime_finder() {
        assert_eq!(prime_finder(10), vec![2, 3, 5, 7]);
    }
}

