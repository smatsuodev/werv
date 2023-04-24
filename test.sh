#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  echo "$input" > tmp/tmp.we
  cargo run --release tmp/tmp.we 2> /dev/null > tmp/tmp.s
  cc -o tmp/func.o -c test/func.c
  cc -o tmp/tmp.o -c tmp/tmp.s
  cc -o tmp/tmp tmp/*.o
  tmp/tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    cargo run --release tmp/tmp.we > tmp/tmp.s
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

assert 0 "print_ok();"
assert 5 "plus2(2, 3)"
assert 9 "plus3(2, 3, 4)"
assert 14 "plus4(2, 3, 4, 5)"
assert 20 "plus5(2, 3, 4, 5, 6)"
assert 27 "plus6(2, 3, 4, 5, 6, 7)"

assert 1 "let one() = 1; one()"
assert 3 "let id(x) = x; id(3)"
assert 5 "let add2(a,b) = a+b; add2(2,3)"
assert 9 "let add3(a,b,c) = a+b+c; add3(2,3,4)"
assert 14 "let add4(a,b,c,d) = a+b+c+d; add4(2,3,4,5)"
assert 20 "let add5(a,b,c,d,e) = a+b+c+d+e; add5(2,3,4,5,6)"
assert 27 "let add6(a,b,c,d,e,f) = a+b+c+d+e+f; add6(2,3,4,5,6,7)"
assert 0 "let fib(n) = if n < 2 n else fib(n-1) + fib(n-2); fib(0)"
assert 1 "let fib(n) = if n < 2 n else fib(n-1) + fib(n-2); fib(1)"
assert 55 "let fib(n) = if n < 2 n else fib(n-1) + fib(n-2); fib(10)"
# assert 1 "let mod(n, modder) = if n < modder n else mod(n-modder, modder); mod(100001, 2)"

echo OK