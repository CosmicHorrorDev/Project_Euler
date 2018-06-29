fn main() {
    let mut number = 600851475143;

    let mut primes = Vec::new();
    primes.push(2);

    loop {
        let current_prime = primes[primes.len() - 1];

        while number % current_prime == 0 {
            number /= current_prime;
        }

        if number == 1 {
            break;
        }

        let next = next_prime(&primes);
        primes.push(next);

    }

    println!("The largest prime factor is {}", primes[primes.len() - 1]);
    println!("{}", (4 as f32).sqrt());
}


fn next_prime(primes: &Vec<u64>) -> u64 {
    let mut potential_prime = primes[primes.len() - 1];

    loop {
        let mut is_prime = true;

        for prime in primes.iter() {
            if potential_prime % prime == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            return potential_prime;
        }

        potential_prime += 1;
    }
}
