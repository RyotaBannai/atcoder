/*
JOI 2017 予選 F L番目のK番目の数
http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0646

長さ N の整数列 a1,a2,…,aN が与えられる。
整数列の連続する部分列のうち長さが K 以上のものそれぞれについて、その中で K
番目に小さい数を書き出していく。
こうして書き出された数のうち L 番目に小さい数を求めよ。
*/
#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int N, K;
  long long L;
  cin >> N >> K >> L;
  vector<int> a(N);
  for (int i = 0; i < N; ++i)
    cin >> a[i];
}