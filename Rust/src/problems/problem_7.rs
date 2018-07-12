use utils::primes::nth_prime;


pub fn solution() -> String {
    let n = 10001;
    let prime = nth_prime(n);
    format!("The {} prime number is {}", n, prime)
}
