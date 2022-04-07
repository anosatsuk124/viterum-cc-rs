#!/usr/bin/env bash
expected="$1"
input="$2"

if [ ! -d ./tmp ]; then
    mkdir ./tmp
fi

echo "$input" | ./target/release/viterum-cc-rs > ./tmp/tmp.S

as -o ./tmp/tmp.o ./tmp/tmp.S
ld -o ./tmp/tmp ./tmp/tmp.o

./tmp/tmp
actual="$?"

if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
    echo OK
    exit 0
else
    echo "$input => expected $expected , but got $actual"
    exit 1
fi