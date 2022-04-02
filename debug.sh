#! /usr/bin/env bash

assert() {
    expected="$1"
    input="$2"

    mkdir ./tmp

    cargo run -- "$input" > ./tmp/tmp.S
    cc -o ./tmp/tmp ./tmp/tmp.S
    ./tmp/tmp
    actual="$?"

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
    else
        echo "$input => $expected expected, but got $actual"
        exit 1
    fi
}

assert 0 0
assert 42 42

echo OK