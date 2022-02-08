#!/bin/bash

try() {
  option=$1

  python3 test_preprocess.py
  cargo run -- $option tmp.vd tmp.s
  if [ "$?" != "0" ]; then
    echo "compiling failed"
    exit 1
  fi
  ${CC:-gcc} tmp.s -o tmp
  ./tmp
  actual=$?
  if [ "$actual" == "0" ]; then
    echo "test: PASSED"
  fi
}

try ""
try "--optimize"
