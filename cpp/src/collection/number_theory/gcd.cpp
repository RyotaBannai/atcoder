#include <iostream>
using namespace std;

/*
大小を気にしなくても、小%大で位置が反転する
a=12, b=15
gcd(15, 12%15) = gcd(15, 12) // 位置が反転
*/
auto gcd2(int a, int b) -> int { return b ? gcd2(b, a % b) : a; }

/*
ループによる最大公約数
再帰でないため、初めに大小を保証
*/
auto gcd2_(int a, int b) -> int
{
  int r;
  if (a < b)
    swap(a, b); // b <= a を保証

  while (b > 0) {
    r = a % b;
    a = b;
    b = r;
  }
  return a;
}

auto gcd3(int a, int b, int c) -> int
{
  int hcf;
  // c<=b<=a
  if (a < b)
    swap(a, b);
  if (b < c) {
    swap(b, c);
    if (c < a)
      swap(a, c);
  }
  // st=a<b?(a<c?a:c):(b<c?b:c);
  for (hcf = c; hcf >= 1; hcf--) // 最小から始める
    if (a % hcf == 0 && b % hcf == 0 && c % hcf == 0)
      break;
  return hcf;
}

/*
l=g*a_*b_
lg=a*b
*/
auto lcm2(int a, int b) -> int
{
  int g = gcd2(a, b);
  int a_ = a / g;
  int b_ = b / g;
  return g * (a_ * b_);
}

/*
3つの場合は、3つの素数の lcm → 先に二つ見て、その lcm と残りを計算
*/
auto lcm3(int a, int b, int c) -> int
{
  int g = gcd3(a, b, c);
  int a_ = a / g;
  int b_ = b / g;
  int c_ = c / g;
  return g * lcm2(lcm2(a_, b_), c_);
}

auto main() -> int
{
  int a, b;
  cin >> a >> b;
  cout << gcd2_(a, b) << endl;
  cout << lcm2(a, b) << endl;
  cout << gcd3(12, 15, 18) << endl;
  cout << lcm3(90, 126, 180) << endl;
}