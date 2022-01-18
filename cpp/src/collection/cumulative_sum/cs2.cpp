/*
AtCoder ABC 084 D - 2017-like Number
https://atcoder.jp/contests/abc084/tasks/abc084_d

N も (N+1)÷2 も素数であるような N を「2017 に似た数」と呼ぶことにします。

Q 個のクエリが与えられます。クエリ i では奇数 li,ri が与えられるので、
li≤x≤ri であって「20172017 に似た数」となっているような奇数 x の個数を求めよ。
*/

#include <iostream>
#include <vector>
using namespace std;

auto main() -> int
{
  // エラトステネスのふるい
  int MAX = 101010;
  vector<int> is_prime(MAX, 1);
  is_prime[0] = 0, is_prime[1] = 0;
  for (int i = 2; i < MAX; ++i) {
    if (!is_prime[i])
      continue;
    for (int j = i * 2; j < MAX; j += i)
      is_prime[j] = 0;
  }

  // 2017-like 数かどうか
  vector<int> a(MAX, 0);
  for (int i = 0; i < MAX; ++i) {
    if (i % 2 == 0)
      continue;
    if (is_prime[i] && is_prime[(i + 1) / 2])
      a[i] = 1;
  }

  // 累積和
  vector<int> s(MAX + 1, 0);
  for (int i = 0; i < MAX; ++i)
    s[i + 1] = s[i] + a[i];

  // クエリ処理
  int Q;
  cin >> Q;
  for (int q = 0; q < Q; ++q) {
    int l, r;
    cin >> l >> r;
    ++r;

    cout << s[r] - s[l] << endl;
  }
}