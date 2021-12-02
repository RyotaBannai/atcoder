/*
遅延評価セグメント木 (Segment Tree with Lazy Propagation)

RMQ の更新・加算の操作は、一点の更新を n 回行うことになるので
O(n logn) だけかかってしまうが、これを
O(log n) でできるように改良したのが遅延評価セグメント木

https://algo-logic.info/segment-tree/

計算量削減のためのメインロジック:
・更新時に無駄な配列の値の更新をしない
=> [a,b) を x で更新する時に、接点の範囲が [a,b)
に完全に入る時はその子供を更新して、その子供より先の値は更新しない.
遅延セグメント木を管理してる配列を dat 、遅延用の配列を lazy だとするとこの dat
更新は更新せずに、完全に収まる節点の子供を lazy に入れる.

・クエリを行うときに dat を更新
クエリの初めに eval の呼び出しにより、lazy に入れていた更新を遅延していた節点を更新
ここで更新した節点の子供へ更新したい値を lazy
へ詰めるため、前に更新が必要だった再起的な更新が、今回のクエリのような必要になった時に更新するように、処理を遅延することができる
*/

#include <iostream>
#include <vector>
using namespace std;
using ll = long long;

template <typename T> struct SegTreeLazy {
  const T INF = numeric_limits<T>::max();
  int n;
  vector<T> dat, lazy;
  SegTreeLazy(int leafs) : dat(leafs * 4, INF), lazy(leafs * 4, INF)
  {
    /*
    必要最低限の最小二分木のメモリを確保 leafs = 7 の時 n = 8 確保するため
    全内部接点は２つの子供を持つ.
    */
    int x = 1;
    while (leafs > x)
      x *= 2;
    n = x;
  }

  auto left(int i) -> int { return i * 2 + 1; }
  auto right(int i) -> int { return i * 2 + 2; }
  auto mid(int l, int r) -> int { return (l + r) / 2; }

  void eval(int k) // 配列のk番目を更新
  {
    if (lazy[k] == INF) // 更新するものが無ければ終了
      return;
    if (k < n - 1) { // 葉でなければ子に伝搬
      lazy[left(k)] = lazy[k];
      lazy[right(k)] = lazy[k];
    }
    dat[k] = lazy[k]; // 自身を更新
    lazy[k] = INF;
  }

  /*
  値の更新には O(log n) だけかかるため、 n 個の更新を行おうとすると O(nlog n) だけかかる
  はじめに n 個の要素を葉にセットしてから、後で同時に更新することで、これを O(n)
  に抑えることが可能.

  i番目は、配列上では i+n-1 番目に格納されている
  */
  void set(int i, T x) { dat[i + n - 1] = x; }
  void build()
  // build := set で直接葉を更新した後に、まとめてセグ木の全体を min で更新
  // n = 8 とした時、n-2 は一番後ろの内部節点なので、そこから根まで順に更新
  {
    for (int k = n - 2; k >= 0; k--)
      dat[k] = fx(dat[2 * k + 1], dat[2 * k + 2]);
  }

  void update(int a, int b, T x, int k, int l, int r)
  {
    eval(k);
    if (a <= l && r <= b) { // 完全に内側の時
      lazy[k] = x;
      eval(k);
    }
    else if (a < r && l < b) { // 一部区間が被る時
      update(a, b, x, left(k), l, mid(l, r));
      update(a, b, x, right(k), mid(l, r), r);
      dat[k] = min(dat[left(k)], dat[right(k)]);
    }
  }
  void update(int a, int b, T x) { update(a, b, x, 0, 0, n); }

  auto query_sub(int a, int b, int k, int l, int r) -> T
  {
    eval(k); // 追加
    if (r <= a || b <= l)
      return INF;
    else if (a <= l && r <= b)
      return dat[k];
    else {
      T vl = query_sub(a, b, left(k), l, mid(l, r));
      T vr = query_sub(a, b, right(k), mid(l, r), r);
      return min(vl, vr);
    }
  }
  auto query(int a, int b) -> T { return query_sub(a, b, 0, 0, n); }

  auto find_rightest(int a, int b, T x) -> int { return find_rightest_sub(a, b, x, 0, 0, n); }
  auto find_leftest(int a, int b, T x) -> int { return find_leftest_sub(a, b, x, 0, 0, n); }
  auto find_rightest_sub(int a, int b, T x, int k, int l, int r) -> int
  {
    // 自分の値が x より大きい or [a,b) が [l,r) の範囲外
    if (dat[k] > x || r <= a || b <= l) {
      return a - 1;
    }
    // 自分が葉ならその位置. dat の index ではなく、葉としての位置
    else if (k >= n - 1) {
      return (k - (n - 1));
    }
    else {
      /*
      一番右の x 以下の値を見たいため、右の部分木を先に見て見つけたらその時点で探索終了.
      左の場合も同様
      */
      int vr = find_rightest_sub(a, b, x, right(k), mid(l, r), r);
      /*
      右の部分木を見て a-1 以外なら return
      [a,b) の範囲で探索しているため、実際に値が見つかった時 a-1
      が帰ることはないため異常値として使用

      左の場合は b を異常値として使用
      */
      if (vr != a - 1) {
        return vr;
      }
      else { // 左の部分木を見て値を return
        return find_rightest_sub(a, b, x, left(k), l, mid(l, r));
      }
    }
  }
  auto find_leftest_sub(int a, int b, T x, int k, int l, int r) -> int
  {
    // 自分の値がxより大きい or [a,b)が[l,r)の範囲外
    if (dat[k] > x || r <= a || b <= l) {
      return b;
    }
    // 自分が葉ならその位置
    else if (k >= n - 1) {
      return (k - (n - 1));
    }
    else {
      int vl = find_leftest_sub(a, b, x, left(k), l, mid(l, r));
      if (vl != b) { // 左の部分木を見て b 以外ならreturn
        return vl;
      }
      else { // 右の部分木を見て値をreturn
        return find_leftest_sub(a, b, x, right(k), mid(l, r), r);
      }
    }
  }

  /* debug */
  inline T operator[](int a) { return query(a, a + 1); }
  void print()
  {
    for (int i = 0; i < 2 * n - 1; ++i) {
      cout << (*this)[i];
      if (i != n)
        cout << ",";
    }
    cout << endl;
  }

  void print_lazy()
  {
    for (auto x : this->lazy)
      cout << x << ", ";
    cout << endl;
  }
};

auto main() -> int
{
  auto ruq = new SegTreeLazy<int>{8};
  ruq->update(3, 7, 2);
  ruq->print();
  // ruq->print_lazy();
}