#!/bin/python3

import sys


def parse_input(text):
    lines = text.split('\n')
    n = int(lines[0])
    numbers = [int(i) for i in lines[1].split()]
    return n, numbers


def main(n, numbers):
    return sum(numbers)


if __name__ == '__main__':
    args = parse_input(sys.stdin.read())
    print(main(*args))