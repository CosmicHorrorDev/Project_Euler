pub fn nth_prime(n: usize) -> usize {
    if n == 1 {
        return 2;
    }

    let mut primes = Vec::with_capacity(n - 1);
    primes.push(3);
    let mut index = 5;
    while primes.len() < n - 1 {
        if prime_sieve(index, &primes) {
            primes.push(index);
        }
        index += 2;
    }

    primes[primes.len() - 1]
}


pub fn prime_below(upper: usize) -> usize {
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
            return false;
        }

        if (num as f32) / (*prime as f32) < (*prime as f32) {
            return true;
        }
    }
    true
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


    #[test]
    fn test_prime_sieve() {
        assert_eq!(prime_sieve(3, &vec![2]), true);
        assert_eq!(prime_sieve(4, &vec![2, 3]), false);
        assert_eq!(prime_sieve(5, &vec![2, 3]), true);
    }


    #[test]
    fn test_prime_below() {
        assert_eq!(prime_below(10), 7);
        assert_eq!(prime_below(100), 97);
    }


    #[test]
    fn test_nth_prime() {
        assert_eq!(nth_prime(1), 2);
        assert_eq!(nth_prime(2), 3);
        assert_eq!(nth_prime(3), 5);
        assert_eq!(nth_prime(4), 7);
    }
}
