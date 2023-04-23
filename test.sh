#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  echo "$input" > tmp.we
  cargo -q run --release tmp.we 2> /dev/null > tmp.s
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    cargo run --release tmp.we 2> /dev/null > tmp.s
    cc -o tmp tmp.s
    ./tmp
    actual="$?"
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 0
assert 42 42
assert 21 '5+20-4'
assert 41 ' 12 + 34 - 5 '
assert 47 '5+6*7'
assert 15 '5*(9-6)'
assert 4 '(3+5)/2'
assert 10 '-10+20'
assert 10 '-(-10)'

assert 0 '0==1'
assert 1 '42==42'
assert 1 '0!=1'
assert 0 '42!=42'

assert 1 '0<1'
assert 0 '1<1'
assert 0 '2<1'
assert 1 '0<=1'
assert 1 '1<=1'
assert 0 '2<=1'

assert 1 '1>0'
assert 0 '1>1'
assert 0 '1>2'
assert 1 '1>=0'
assert 1 '1>=1'
assert 0 '1>=2'

assert 1 'a=1;a'
assert 2 'a=1;b=2;b'
assert 1 'a=1;b=2;a'
assert 3 'a=1;b=2;a+b'

assert 6 "foo = 1; bar = 2 + 3; foo + bar"

assert 6 "foo = 1; bar = 2 + 3; return foo + bar;"

assert 10 "a = 1; if a 10";
assert 20 "a = 0; if a 10 else 20";
assert 20 "a = 0; b = 1; if a 10 else if b 20 else 30";
assert 30 "a = 0; b = 0; if a 10 else if b 20 else 30";
assert 10 "a = 0; if a == 0 10 else 20";

assert 10 "{10}"
assert 10 "{{{10}}}"
assert 20 "{10}; 20"
assert 10 "{return 10;}; 20"
assert 11 "if 1 { 10; 10 + 1 } else { 20; 10 }"
assert 10 "if 0 { 10; 10 + 1 } else { 20; 10 }"

echo OK