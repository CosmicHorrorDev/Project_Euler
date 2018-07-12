## Problem 4 Reasoning

Problem 4 is asking you to find the largest palindrome product of 2 3-digit numbers. One of the most time consuming parts is finding out which palindrome product is the largest. It would be most efficent to start with the values that give the largest product, then work our way through them till one of the products is a palindrome, but how do we determine this descending order for the products?

### Multiplication Tables

The way I thought of this is with a multiplication table. For example here is a slice of one.

| | 9 | 8 | 7 | 6 |
| --: | --: | --: | --: | --: |
| 9 | 81 |
| 8 | 72 | 64 |
| 7 | 63 | 56 | 49 |
| 6 | 54 | 48 | 42 | 36 |

With the order from largest to smallest being marked below.

| | 9 | 8 | 7 | 6 |
| --: | --: | --: | --: | --: |
| 9 | 1 |
| 8 | 2 | 3 |
| 7 | 4 | 5 | 7 |
| 6 | 6 | 8 | 9 | 10 |

So it starts at 9 * 9, then moves down then right repeatedly, and after each of those movements it slides down the entire diagonal going down and to the left. And because this pattern holds true for all positive multiplication tables we can follow it and the first palindrome product we find will already be the largest!

## Optimizations

The main slow part after implementing the above reasoning was checking if the product is a palindrome. In python I do this by casting the product to a string, then checking if the string is equal to the reversed string.

```
str(product) == str(product)[::-1]
```

Yes this is a slow way of doing it, but it's so concise. In rust I was trying to be a bit more focused on speed, so I initailly was splitting the number into a `Vec<usize>`
of all the digits. Then I would compare, the vector with the reverse vector to see if it was a palindrome.

```
vec.eq(vec.rev())
```

This is nice and concise, but checking if a number was a palindrome was the main thing eating my cycles. I made a couple small improvements like keeping the vector reversed after splitting it into digits or using a `u8` instead of `usize` to store the digits. These both had small preformance boosts, but I finally came down to a nice, fast way of checking. To check now I subtract the number from the digit in the ones place multiplied up to the magnitude of the highest digit. Then I do a floored division and repeat. If the number is ever larger than the manitude of the highest digit or less than zero after the subtraction, then it is not a palindrome. This allows me to return false for the first pair of digits I see that don't match, and avoids creating lots of vectors. For example:

```
Starting number 2342
2342 - (2 * 1000) = 2342 - 2000 = 342
342 < 1000 && 342 > 0 //so far so good
floor(342 / 10) = 34

34 - (4 * 10) = 34 - 40 = -16
-16 < 10 && -16 > 0 is false so it is not a palindrome
```

#### Benchmarks

So how do these changes compare, well here is the gist of the two methods I stuck with the longest. Splitting to a reverse vec of usize and the newest change.

| Method | Mean ± σ [µs] |
|:---|---:|
| Split to Vec | 174.97 ± 5.52 |
| Compare in Place | 14.09 ± 1.51 |

