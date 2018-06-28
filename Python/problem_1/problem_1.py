#!/usr/bin/env python3

def main(UPPERBOUND):
    sum = 0

    for i in range(1, UPPERBOUND):
        if not i % 3 and not i % 5:
            sum += i

    return sum

if __name__ == '__main__':
    print(main(1000))

