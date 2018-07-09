#!/usr/bin/env python3


def solution():
    UPPER = 1000
    num1, num2 = largest_palindrome_product(UPPER)
    product = num1 * num2
    return f'The largest palindrome product is {product} = {num1} * {num2}'


def largest_palindrome_product(UPPER):
    num1 = UPPER + 1
    num2 = num1

    while num2 > 0:
        slide1 = num1
        slide2 = num2

        palindrome = False
        while slide1 < UPPER and slide2 > 0:
            product = slide1 * slide2
            palindrome = str(product) == str(product)[::-1]
            if palindrome:
                break

            slide1 += 1
            slide2 -= 1

        if palindrome:
            break

        if num1 > num2:
            num1 -= 1
        else:
            num2 -= 1

    return slide1, slide2

<<<<<<< HEAD:Problems/Problem_4/Python/problem_4.py

if __name__ == '__main__':
    slide1, slide2 = largest_palindrome_product(1000)
    product = slide1 * slide2
    print(f'The largest parlindrome product is {product} = {slide1} * {slide2}')

=======
>>>>>>> refactorProblemRunners:Python/Problems/problem_4.py
