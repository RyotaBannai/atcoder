/*
AtCoder AGC 023 A - Zero-Sum Ranges
https://atcoder.jp/contests/agc023/tasks/agc023_a

長さ N の整数列 a0,a1,…,aN−1 が与えられる。
この整数列の連続する区間であって、その区間内の値の総和が 0 になるものが何個あるか答えよ。
*/
#include <iostream>
#include <map>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int N; // or ll
  cin >> N;

  vector<ll> a(N);
  for (int i = 0; i < N; i++)
    cin >> a[i];

  vector<ll> s(N + 1, 0);
  map<ll, ll> nums;
  for (int i = 0; i < N; i++)
    s[i + 1] = s[i] + a[i];
  for (int i = 0; i < N + 1; i++)
    nums[s[i]]++;

  ll ans = 0;
  for (auto it : nums) {
    ll num = it.second; // it.first (ある index i における累積和) になる index の合計が it.second 個
    ans += num * (num - 1) / 2; // 組合せ N 個の中から K 個取り出す方法. 今回は K=2
                                // で左右の終端の位置を取り出して作る区間が何通りあるかを考える
  }

  cout << ans << endl;
}