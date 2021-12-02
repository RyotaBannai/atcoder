/*
セグメント木 （Segment Tree）

RMQ (Range Minimam Query), RUQ（Range Update Query）
https://algo-logic.info/segment-tree/

それぞれへのアクセス:

左の子：dat[2*i+1]
右の子：dat[2*i+2]

親：dat[(i-1)/2]
*/
#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

/* RMQ：[0,n-1] について、区間ごとの最小値を管理する構造体
    update(i,x): i 番目の要素を x に更新。O(log(n))
    query(a,b): [a,b) での最小の要素を取得。O(log(n))
*/
template <typename T> struct RMQ {
  const T INF = numeric_limits<T>::max();
  int n = 8;     // 葉の数
  vector<T> dat; // 管理するノード
  auto left(int i) -> int { return dat[i * 2 + 1]; }
  auto right(int i) -> int { return dat[i * 2 + 2]; }

  void update(int i, int v) // i := 更新したい数列の位置　v := 更新する値
  {
    i += n - 1; // i番目は、配列上では n-1+i 番目に格納されている
    dat[i] = v; // 葉の更新
    while (i > 0) {
      // 更新した子供が左右どちらかはわからないため、親を見つけてその子供二つ見つける
      i = (i - 1) / 2;
      // 子の最小値で親ノードの値も更新
      dat[i] = min(left(i), right(i));
    }
  }

  // k := 現在見ているノードの位置  [l,r) := dat[k]が表している区間
  auto query_sub(int a, int b, int k, int l, int r) -> T
  {
    if (r <= a || b <= l) // 範囲外なら考えない
      return INF;
    else if (a <= l && r <= b) // 範囲内なので自身の値を返す（子供の最小値を確認するまでもない）
      return dat[k];
    else { // 一部区間が被る時
      T vl = query_sub(a, b, k * 2 + 1, l, (l + r) / 2);
      T vr = query_sub(a, b, k * 2 + 2, (l + r) / 2, r);
      return min(vl, vr);
    }
  }
  auto query(int a, int b) -> T { return query_sub(a, b, 0, 0, n); }
};

auto main() -> int { return 0; }