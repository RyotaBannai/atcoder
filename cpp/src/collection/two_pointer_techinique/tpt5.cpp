/*
ARC 022 B 細長いお菓子
https://atcoder.jp/contests/arc022/tasks/arc022_2

長さ n の正の整数列 a1,a2,…,an が与えられる。
整数列の連続する部分列のうち、「同じ数値が 2 箇所以上登場しない」という条件を満たす最大長を求めよ。
*/
#include <iostream>
#include <set>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int n;
  cin >> n;
  vector<ll> a(n);
  for (int i = 0; i < n; i++) {
    cin >> a[i];
  }

  ll res = 0;
  int right = 0;
  for (int left = 0; left < n; left++) {
    while (right < n && (right == left || a[right - 1] < a[right])) {
      right++;
    }

    res += right - left;
  }

  cout << res << endl;
}