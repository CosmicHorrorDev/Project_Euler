#!/usr/bin/env python3


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


if __name__ == '__main__':
    slide1, slide2 = largest_palindrome_product(1000)
    product = slide1 * slide2
    print(f'The largest parlindrome product is {product} = {slide1} * {slide2}')

