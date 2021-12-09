/*
Binary Indexed Tree(BIT)

References:
https://www.hackerearth.com/practice/notes/binary-indexed-tree-or-fenwick-tree/
https://algo-logic.info/binary-indexed-tree/#toc_id_1_4
*/

#include <iostream>
#include <vector>
using namespace std;

/* BIT: 区間和の更新や計算を行う構造体
    初期値は a_1 = a_2 = ... = a_n = 0
    ・add(i,x): a_i += x とする
    ・sum(i): a_1 + a_2 + ... + a_i を計算する
    計算量は全て O(logn)
*/
template <typename T> struct BIT {
  int n;
  vector<T> bit;
  BIT(int leafs, int defa = 0) : n(leafs + 1), bit(n, defa) {}

  /*
  i と　x が与えられた時、ai に x を加算する
  = i から初めて、i に最後に 1 が立っているビットを加算しながら、最後の i の値に x を加える　
  */
  void add(int i, T x)
  {
    for (int idx = i; idx < n; idx += (idx & -idx)) {
      bit[idx] += x;
    }
  }

  /*
  i から初めて最後に 1 が立っているビットを減算しながら、i の値を合計していく
  */
  T sum(int i)
  {
    T s(0);
    for (int idx = i; idx > 0; idx -= (idx & -idx)) {
      s += bit[idx];
    }
    return s;
  }

  // BIT の二分探索木
  auto lower_bound(T w) -> int
  {
    if (w <= 0) {
      return 0;
    }
    else {
      int x = 0, r = 1;
      while (r < n)
        r = r << 1;
      // 長さ len は１だん下がるごとに半分
      for (int len = r; len > 0; len = len >> 1) {
        // 採用するとき
        if (x + len < n && bit[x + len] < w) {
          w -= bit[x + len];
          x += len;
        }
      }
      return x + 1;
    }
  }
  inline T operator[](int i) { return bit[i]; }
};

void test()
{
  // セグ木と同様に build するか、add 経由で内部節点を更新してから sum を使う
  auto bit = make_unique<BIT<int>>(8, 2);
  cout << bit->lower_bound(2) << endl;
  cout << bit->lower_bound(3) << endl;
  // i<=2 -> 全て 2 なので 1
  // i>=3 -> 9 末尾+1
}

// BIT を集合として使う
void set()
{
  // 要素 1~4 を管理
  int leafs = 8;
  auto bit = make_unique<BIT<int>>(leafs, 0);
  // 要素 1, 2 を追加 (=1 を立てて要素があるとみなす)
  bit->add(2, 1);
  bit->add(3, 1);
  bit->add(5, 1);
  /*
  要素 2 の順番(何番目に小さいか)は 0~2 区間の 1 の和で判別
  */
  cout << bit->sum(2) << endl;
  // cout << (*bit)[4] << endl;
  // i 番目に小さい要素 a. 以下では 1 番目に小さい要素は 2 となる.
  cout << bit->lower_bound(1) << endl;
}

auto main() -> int { test(); }