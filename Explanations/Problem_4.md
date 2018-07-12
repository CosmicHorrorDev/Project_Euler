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
