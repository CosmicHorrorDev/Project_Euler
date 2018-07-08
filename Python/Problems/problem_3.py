#!/usr/bin/env python3


def solution():
    NUM = 600851475143
    prime = largest_prime_factor(NUM)
    return f'The largest prime factor of {NUM} is {prime}'


def largest_prime_factor(number):
    assert(number > 1)
    index = 3

    while not number % 2:
        number /= 2

    while True:
        # Reduce given number
        while not number % index:
            number /= index

        # Number is fully reduced, we are done
        if number == 1:
            break

        # Get next prime
        index += 2

    return index

