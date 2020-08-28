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

try 0 "func main() { return 0 }"
try 42 "func main() { return 42 }"

try 3 "func main() { return 1 + 2 }"
try 6 "func main() { return 1 + 2 + 3 }"
try 5 "func main() { return 6 - 1 }"
try 17 "func main() { return 20 - 5 + 2 }"

try 20 "func main() { return 2 * 2 * 5 }"
try 5 "func main() { return 20 / 4 }"
try 12 "func main() { return 1 + 2 * 3 + 5 / 1 }"

try 38 "func main() { return 3 + 5 * 7 }"
try 56 "func main() { return (3 + 5) * 7 }"
try 1 "func main() { return ((1)) }"

try 2 "func main() { return --+---++-2 }"
try 3 "func main() { return -3 * -1 }"

try 0 "func main() { return 1 & 0 }"
try 1 "func main() { return 1 & 1 }"
try 0 "func main() { return 0 | 0 }"
try 1 "func main() { return 1 | 0 }"
try 0 "func main() { return 1 ^ 1 }"
try 1 "func main() { return 1 ^ 0 }"


try 0 "func main() { return 0 == 1 }"
try 1 "func main() { return 1 == 1 }"
try 1 "func main() { return 0 != 1 }"
try 0 "func main() { return 1 != 1 }"

try 0 "func main() { return 1 < 1 }"
try 1 "func main() { return 0 < 1 }"
try 0 "func main() { return 2 <= 1 }"
try 1 "func main() { return 1 <= 1 }"
try 0 "func main() { return 1 > 1 }"
try 1 "func main() { return 2 > 1 }"
try 0 "func main() { return 0 >= 1 }"
try 1 "func main() { return 1 >= 1 }"

try 1 "func main() {
  if 1 == 1 {
    return 1
  } else {
    return 0
  }
}"

try 0 "func main() {
  if 0 == 1 {
    return 1
  } else {
    return 0
  }
}"

try 2 "func main() {
  if 1 == 1 {
    if 1 == 1 {
      return 2
    }
    return 1
  } else {
    return 0
  }
}"

try 1 "func main() {
  if 1 == 1 {
    if 0 == 1 {
      return 2
    }
    return 1
  } else {
    return 0
  }
}"

try 10 "func main() {
  var a = 10
  return a
}"

try 15 "func main() {
  var a = 10
  var b = 5
  return a + b
}"

try 0 "func main() {
  var a = 10
  var b = a
  a = -10
  return a + b
}"