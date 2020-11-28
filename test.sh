#!/bin/bash

try() {
  option=$1

  cargo run -- $option test.vd tmp.s 2> /dev/null
  gcc tmp.s -o tmp
  ./tmp
  actual=$?
  if [ "$actual" != "0" ]; then
    echo "test: FAILED"
  else
    echo "test: PASSED"
  fi
}

try ""
try "--optimize"
