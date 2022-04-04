#!/usr/bin/env bash
expected="$1"
input="$2"

if [ ! -d ./tmp ]; then
    mkdir ./tmp
fi

echo "$input" | ./target/release/viterum-cc-rs > ./tmp/tmp.S

cc -o ./tmp/tmp ./tmp/tmp.S

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