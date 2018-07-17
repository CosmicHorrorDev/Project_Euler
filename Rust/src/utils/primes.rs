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
    while index < upper {
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
        let mut primes = vec![2];
        for i in 3..100 {
            if prime_sieve(i, &primes) {
                primes.push(i);
            }
        }

        // Check if prime sieve built the right list
        assert_eq!(primes, primes_helper());
    }


    #[test]
    fn test_prime_below() {
        let known_primes = primes_helper();

        for i in 3..100 {
            let lower_primes = known_primes.iter()
                                           .filter(|&p| *p < i)
                                           .collect::<Vec<_>>();
            let highest_prime = lower_primes[lower_primes.len() - 1];

            println!("{}", i);
            assert_eq!(prime_below(i), *highest_prime);
        }
    }


    #[test]
    fn test_nth_prime() {
        let known_primes = primes_helper();

        for (i, &prime) in known_primes.iter().enumerate() {
            assert_eq!(nth_prime(i + 1), prime);
        }
    }
}
