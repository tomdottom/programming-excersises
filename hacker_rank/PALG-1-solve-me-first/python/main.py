#!/bin/python3

import sys


def parse_input(text):
    lines = text.split('\n')
    x = int(lines[0])
    y = int(lines[1])
    return x, y


def main(x, y):
    return x + y


if __name__ == '__main__':
    args = parse_input(sys.stdin.read())
    print(main(*args))