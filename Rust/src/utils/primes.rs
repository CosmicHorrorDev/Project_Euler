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

    fn primes_helper() -> Vec<usize> {
        vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59,
        61, 67, 71, 73, 79, 83, 89, 97]
    }

    #[test]
    fn test_is_prime() {
        // Build vec of primes with index for reference
        let mut i = 0;
        let known_primes = primes_helper();

        for check in 2..98 {
            if check == known_primes[i] {
                // check should be a prime number, inc to next known_prime
                assert!(is_prime(check));
                i += 1;
            } else {
                // check is not prime, keep working on current known_prime
                assert!(!is_prime(check));
            }
        }
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
