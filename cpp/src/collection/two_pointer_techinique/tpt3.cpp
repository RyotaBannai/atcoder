/*
ABC 032 C 列
https://atcoder.jp/contests/abc032/tasks/abc032_c

長さ n の整数列 a1,a2,…,an と整数 K が与えられる。
整数列の連続する部分列で、その積が K 以下となるもののうち、最大の長さを求めよ
(条件を満たす区間がないときは 0 を出力)。
*/
#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int n;
  ll K;
  cin >> n >> K;
  vector<ll> a(n);
  for (int i = 0; i < n; i++) {
    cin >> a[i];
    if (a[i] == 0) { // 0 ならどれだけ積を計算しても K 以下になるため最大値 n となる
      cout << n << endl;
      exit(0);
    }
  }

  int res = 0;
  ll mul = 1; // 数値が大きくなるため int でなく long long で宣言
  int right = 0;
  for (int left = 0; left < n; left++) {
    while (right < n && mul * a[right] <= K) {
      mul *= a[right];
      right++;
    }

    /* break した状態で right は条件を満たす最大 */
    res = max(res, right - left);

    /* left をインクリメントする準備 */
    if (right == left)
      right++; // right が left に重なったら right も動かす
    else
      mul /= a[left];
  }

  cout << res << endl;
}