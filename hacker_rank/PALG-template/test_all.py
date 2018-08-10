#!/bin/python3
from subprocess import check_output
from textwrap import dedent

tests = [
    ('', 'Foo\n'),
]


def test(cmd):
    print('Testing: {}'.format(cmd))
    for index, (input, expected_output) in enumerate(tests):
        output = check_output(
            cmd,
            input=input.encode('utf-8'),
            shell=True,
        ).decode('utf-8')
        if output == expected_output:
            print('Test {} passed'.format(index))
        else:
            print('Test {} failed'.format(index))
            print(
                dedent('''
                Input:
                {}
                Expected {}
                Got {}
                ''').format(input, expected_output, output))


test('python3 ./python/main.py')
test('go run ./go/main.go')
test('cd rust && cargo run')