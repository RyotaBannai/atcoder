/*
ABC 017 D サプリメント
https://beta.atcoder.jp/contests/abc017/tasks/abc017_4

1 以上 M 以下の整数からなる長さ N の数列 f1,f2,…,fn が与えられる。
数列を前から順番に区切って行く。各区間は「同じ値が二度以上登場しない」という条件を満たすようにしたい。
そのような方法が何通りあるかを 1000000007 で割った余りで求めよ。
*/
#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int n, m;
  cin >> n >> m;
  vector<ll> a(n);
  for (int i = 0; i < n; i++) {
    cin >> a[i];
  }

  ll res = 0;
  int right = 0;
  vector<ll> is;
  for (int left = 0; left < n; left++) {
    while (right < n) {}

    if (right == left)
      right++;

    res += right - left;
  }

  cout << res << endl;
}