pub fn nth_prime(n: usize) -> usize {
    let mut primes = vec![2];
    let mut index = 3;
    while primes.len() < n {
        if prime_sieve(index, &primes) {
            primes.push(index);
        }
        index += 2;
    }

    primes[primes.len() - 1]
}


pub fn largest_prime(upper: usize) -> usize {
    let mut primes = vec![2];
    let mut index = 3;
    while index <= upper {
        if prime_sieve(index, &primes) {
            primes.push(index);
        }
        index += 2;
    }

    primes[primes.len() - 1]
}


pub fn is_prime(num: usize) -> bool {
    if num % 2 == 0 && num > 2 {
        return false;
    }

    let mut index = 3;
    while index < num {
        if num % index == 0 {
            return false;
        }
        index += 2;
    }

    true
}


fn prime_sieve(num: usize, primes: &[usize]) -> bool {
    for prime in primes {
        if num % prime == 0 {
            return true;
        }
    }
    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
    }
}
