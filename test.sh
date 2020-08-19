#!/bin/bash

try() {
  expected=$1
  source=$2

  echo "$source" > tmp.vd
  cargo run tmp.vd tmp.s 2> /dev/null
  gcc -m32 tmp.s -o tmp
  ./tmp
  actual=$?
  if [ "$actual" != "$expected" ]; then
    echo "$source => $expected expected, but got $actual"
    exit 1
  else
    echo "$source => $expected"
  fi
}

try 0 "0"
try 42 "42"

try 3 "1 + 2"
try 6 "1 + 2 + 3"
try 5 "6 - 1"
try 17 "20 - 5 + 2"

try 20 "2 * 2 * 5"
try 5 "20 / 4"
try 12 "1 + 2 * 3 + 5 / 1"

try 38 "3 + 5 * 7"
try 56 "(3 + 5) * 7"
try 1 "((1))"
