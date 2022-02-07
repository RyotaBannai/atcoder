#include <iostream>
using namespace std;
using ll = long long;
const int MOD = 1000000007;
/*
>>> -17 % 5
-2

あまりを求めた結果が負になったら法 10000000071000000007 を足す
*/

// 負の数にも対応した % 演算
auto mod(ll val, ll m) -> ll
{
  ll res = val % m;
  if (res < 0)
    res += m;
  return res;
}

auto main() -> int
{
  int a = 2000000020;
  int b = 20;

  cout << "test: " << (a % MOD) << endl; // 6

  cout << "普通に計算して余りを求める: " << (a - b) % MOD << endl;
  cout << "余り求めてから計算して余りを求める: " << ((a % MOD) - (b % MOD)) % MOD << endl;
  cout << "余り求めてから計算して余りを求める (対策済): " << mod((a % MOD) - (b % MOD), MOD)
       << endl;
}