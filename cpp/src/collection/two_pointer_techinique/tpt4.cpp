/*
ABC 038 C 単調増加
https://beta.atcoder.jp/contests/abc038/tasks/abc038_c

長さ n の正の整数列 a1,a2,…,an が与えられる。
整数列の連続する部分列のうち、単調増加となっているものを数え上げよ。
*/
#include <iostream>
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
    /** right と left が重なった時(最大値要素だけの部分集合分を res に加えた後)は、
     * inc して次の最大値まで広げる */
    while (right < n && (right == left || a[right - 1] < a[right])) {
      right++;
    }

    // 部分集合をまとめて加える i.g. 1 2 3 なら 1, 1 2, 1 2 3 の３つの部分集合
    res += right - left;
  }

  cout << res << endl;
}