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

  int res = 0;
  int right = 0;
  set<ll> s;
  for (int left = 0; left < n; left++) {
    while (right < n && s.find(a[right]) == s.end()) {
      s.insert(a[right]);
      right++;
    }

    res = max(res, (int)s.size());

    if (right == left)
      right++;
    else
      s.erase(a[left]);
  }

  cout << res << endl;
}