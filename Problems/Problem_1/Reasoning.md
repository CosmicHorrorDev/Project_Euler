## Problem 1 Reasoning

So problem 1 is very simple so I'll avoid the big explanation, but I was trying to figure out the fastest way to solve it logically. The naive approach (albeit a lot simpler and easier to read) is to loop over the values from 1 to 100 and add any values that are divisible by 3 or 5. So lets consider just up to 15 for simplicity, all we are doing is this

```
3 + 5 + 6 + 9 + 10 + 15 = 38
```

but is there a faster way to do it? Well we can manipulate it around to get

```
(5 + 10 + 15) + (3 + 6 + 9 + 15) - 15 = 23
5(1 + 2) + 3(1 + 2 + 3) - 15(1)= 23
```

which shows that all we are doing is 5 times the sum of values from 0 to floor(10 / 5) plus 3 times the sum of values from 0 to floor(10 / 3). However if we want to do these sums separately then we must subtract the sum of values divisible by 15 in the same way since they would be counted twice. So this is nice since now we could just sum the values from 0 to floor(1000 / 3) and get the values from 0 to floor(1000 / 5) and floor(1000 / 15) along the way which should be faster, but wait there's more. We can actually get the sum of values from 0 to a value with a nice little trick. I think it's a story about Euler as a kid or something like that but here's the gist. If we want to sum the values from 0 to 6

```
0 1 2 3 4 5 6
```

we can add them all up (0 + ... + 6) or we can see that the sum of the first and last value is 6 (0 + 6), and likewise the second and second to last is also 6, with this trend continuing till the center where we have a 3 by itself which is conviently 6 / 2. So I'll just give the general technique to calculate the sum from 0 to NUMBER with some psuedo-code.

```
sum_vals = floor(NUMBER / 2) * NUMBER

if NUMBER % 2 == 0:
    sum_vals += NUMBER / 2
```

So there you have it, now we can find the sum of values that are multiples of 3 or 5 up to 1000 without having to loop at all! This makes it fast if we need to find the sum up to some arbitrarily large number, but makes things quite complicated for if you want to get the sum of multiples of something like 3, 5, 7, 11, 13 ... which is something I do want to add in the ability to do later on.
