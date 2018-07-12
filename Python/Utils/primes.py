#! /usr/bin/env python
def prime_range(upper):
    assert upper > 1, 'Numbers below 2 aren\'t prime dummy'

    primes = [2]
    for i in range(3, upper + 1, 2):
        if prime_sieve(i, primes):
            primes.append(i)

    return primes


def is_prime(num):
    pass


def prime_sieve(num, primes):
    prime = True
    for prime in primes:
        if num % prime == 0:
            prime = False

    return prime


if __name__ == '__main__':
    pass
    # Do unit tests and junk here?

