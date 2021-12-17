/*
2 Dimentional Binary Indexed Tree(BIT)

BIT2D:
初期値は全て 0
・add(h,w,x): (h,w) に x を加算する
・sum(h,w): 1≦i≦h かつ 1≦j≦w の範囲の合計値を求める
・query(h1,w1,h2,w2): h1≦i<h2 かつ w1≦j<w2 の範囲の合計値を求める(1-indexed)
計算量は全て O(logW * logH)


基本的には、
BIT を 2 重ループにして処理する行列
*/

#include <iostream>
#include <vector>
using namespace std;
template <typename T> struct BIT2D {
  int H, W;
  vector<vector<T>> bit;
  BIT2D(int H_, int W_, int defa = 0) { init(H_, W_, defa); }
  void init(int H_, int W_, int defa)
  {
    H = H_ + 1;
    W = W_ + 1;
    bit.assign(H, vector<T>(W, defa));
  }

  // 1≦i≦h かつ 1≦j≦w
  // 一点に値を加える
  void add(int h, int w, T x)
  {
    for (int i = h; i < H; i += (i & -i)) { // 加えていくから制限が H より小さい
      for (int j = w; j < W; j += (j & -j)) {
        bit[i][j] += x;
      }
    }
  }

  // 1≦i≦h かつ 1≦j≦w
  auto sum(int h, int w) -> T
  {
    T s(0);
    for (int i = h; i > 0; i -= (i & -i)) { // 引いていくから制限が 0 より大きい
      for (int j = w; j > 0; j -= (j & -j)) {
        s += bit[i][j];
      }
    }
    return s;
  }

  /*
  h1≦i<h2 かつ w1≦j<w2

  H, W ともに最大の区間を初めに求め、
  それぞれの小さい方を引く.
  ただし、H, W の小さい区間を二度引いてしまうため、１回分足す.
  */
  auto query(int h1, int w1, int h2, int w2) -> T
  {
    return sum(h2 - 1, w2 - 1) - sum(h2 - 1, w1 - 1) - sum(h1 - 1, w2 - 1) + sum(h1 - 1, w1 - 1);
  }
};

void test()
{
  auto bit = make_unique<BIT2D<int>>(6, 6);
  bit->add(3, 3, 5);
  bit->add(3, 5, 10); // 4 の外側にあるため加味されない
  cout << bit->sum(4, 4) << endl;
}

auto main() -> int { test(); }