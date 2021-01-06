#!/bin/bash

try() {
  option=$1

  cargo run -- $option test.vd tmp.s
  if [ "$?" != "0" ]; then
    echo "compiling failed"
    exit 1
  fi
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
