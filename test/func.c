#include <stdio.h>

void print_ok() { printf("OK\n"); }

void putnbr(int n) { printf("%d\n", n); }

int plus2(int a, int b) { return a + b; }
int plus3(int a, int b, int c) { return a + b + c; }
int plus4(int a, int b, int c, int d) { return a + b + c + d; }
int plus5(int a, int b, int c, int d, int e) { return a + b + c + d + e; }
int plus6(int a, int b, int c, int d, int e, int f) {
  return a + b + c + d + e + f;
}