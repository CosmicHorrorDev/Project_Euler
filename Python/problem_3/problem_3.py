#!/usr/bin/env python3

# The logic behind this is that we can continue breaking up the large number
# by finding the primes it is divisible by, the tricky part is this involes
# finding all the smaller prime factors to find the largest


def main(number):
    assert(number > 1)
    primes = [2]

    while True:
        current_prime = primes[-1]

        # Reduce given number
        while not number % current_prime:
            number /= current_prime

        # Number is fully reduced, we are done
        if number == 1:
            break

        # Get next prime
        primes.append(next_prime(primes))

    return current_prime


def next_prime(primes):
    '''Finds primes by using a given list of primes
    and increments a test number till it is not divisible
    by any of the primes in the list

    Note: may be able to optimize by only checking the primes under the sqrt
    of the test value, benchmark later for it'''
    potential_prime = primes[-1] + 1

    while True:
        is_prime = True

        for prime in primes:
            if not potential_prime % prime:
                is_prime = False
                break

        if is_prime:
            return potential_prime

        potential_prime += 1


if __name__ == '__main__':
    number = 600851475143
    answer = main(number)
    print(f'The largest prime factor of {number} is {answer}')


