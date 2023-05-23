#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  echo "$input" > tmp/tmp.we
  cargo run --release tmp/tmp.we 2> /dev/null > tmp/tmp.s
  # cargo run --release tmp/tmp.we > tmp/tmp.s
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

assert 1 'let a: int = 1; a'
assert 2 'let a: int = 1; let b: int = 2; b'
assert 1 'let a: int = 1; let b: int = 2; a'
assert 3 'let a: int = 1;let b: int = 2; a+b'
assert 4 'let a: int = 1; a=2; a+a'

assert 6 "let foo: int = 1; let bar: int = 2 + 3; foo + bar"

assert 6 "let foo: int = 1; let bar: int = 2 + 3; return foo + bar;"

assert 10 "let a: int = 1; if a 10";
assert 20 "let a: int = 0; if a 10 else 20";
assert 20 "let a: int = 0; let b: int = 1; if a 10 else if b 20 else 30";
assert 30 "let a: int = 0; let b: int = 0; if a 10 else if b 20 else 30";
assert 10 "let a: int = 0; if a == 0 10 else 20";

assert 10 "{10}"
assert 10 "{{{10}}}"
assert 20 "{10}; 20"
assert 10 "{return 10;}; 20"
assert 11 "if 1 { 10; 10 + 1 } else { 20; 10 }"
assert 10 "if 0 { 10; 10 + 1 } else { 20; 10 }"

# assert 0 "print_ok();"
# assert 5 "plus2(2, 3)"
# assert 9 "plus3(2, 3, 4)"
# assert 14 "plus4(2, 3, 4, 5)"
# assert 20 "plus5(2, 3, 4, 5, 6)"
# assert 27 "plus6(2, 3, 4, 5, 6, 7)"

assert 1 "let one(): int = 1; one()"
assert 3 "let id(x: int): int = x; id(3)"
assert 5 "let add2(a: int,b: int): int = a+b; add2(2,3)"
assert 9 "let add3(a: int,b: int,c: int): int = a+b+c; add3(2,3,4)"
assert 14 "let add4(a: int,b: int,c: int,d: int): int = a+b+c+d; add4(2,3,4,5)"
assert 20 "let add5(a: int,b: int,c: int,d: int,e: int): int = a+b+c+d+e; add5(2,3,4,5,6)"
assert 27 "let add6(a: int,b: int,c: int,d: int,e: int,f: int): int = a+b+c+d+e+f; add6(2,3,4,5,6,7)"
assert 0 "let fib(n: int): int  = if n < 2 n else fib(n-1) + fib(n-2); fib(0)"
assert 1 "let fib(n: int): int = if n < 2 n else fib(n-1) + fib(n-2); fib(1)"
assert 55 "let fib(n: int): int = if n < 2 n else fib(n-1) + fib(n-2); fib(10)"
assert 1 "let mod(n: int, modder: int): int = if n < modder n else mod(n-modder, modder); mod(100001, 2)"
assert 4 "let mod(n: int, modder: int): int = if n < modder n else mod(n-modder, modder); mod(100004, 5)"
assert 5 "let a: int = 4; a = 5; a"
assert 5 "let a: int; a = 5; a"

assert 10 "let a: int = 10; let b: *int = &a; *b"
assert 3 "let a: int = 3; let b: int = 5; let c: *int = &b + 1; *c"
assert 10 "let a: int = 10; let b: *int = &a; let c: **int = &b; let d: ***int = &c; ***d"
assert 15 "let a: int = 10; let b: *int = &a; let c: int = 5; let d: *int = &c; *b+*d"

assert 3 "let x: int = 0;
let y: *int = 0;
y = &x;
*y = 3;
return x;
"

echo OK