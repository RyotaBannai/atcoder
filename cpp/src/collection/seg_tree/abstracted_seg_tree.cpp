#include <iostream>
#include <vector>
using namespace std;

using X = int;
using M = int;

auto fx = [](X x1, X x2) -> X { return min(x1, x2); };
auto fa = [](X x, M m) -> X { return m; };
auto fm = [](M m1, M m2) -> M { return m2; };
// auto fp = [](M m, long long n) -> M { return m * n; };
int ex = numeric_limits<int>::max();
int em = numeric_limits<int>::max();

/*
  SegTreeLazy<X,M>(n,fx,fa,fm,ex,em):
  モノイド(集合 X, 二項演算 fx, fa, fm, 単位元 ex, em)についてサイズ n で構築

  fx: X × X → X
  fa: X × M → X
  fm: M × M → M
  ex: モノイドXでの単位元
  em: モノイドMでの単位元

  fa, fm は

  最小値だけではなく max など外部から評価方法を渡せるようにする
*/

template <typename X, typename M> struct SegTreeLazy {
  using FX = function<X(X, X)>;
  using FA = function<X(X, M)>;
  using FM = function<M(M, M)>;
  int n;
  FX fx;
  FA fa;
  FM fm;
  const X ex;
  const M em;
  vector<X> dat;
  vector<M> lazy;
  SegTreeLazy(int n_, FX fx_, FA fa_, FM fm_, X ex_, M em_)
      : fx(fx_), fa(fa_), fm(fm_), ex(ex_), em(em_), dat(n_ * 4, ex), lazy(n_ * 4, em)
  {
    int x = 1;
    while (n_ > x)
      x *= 2;
    n = x;
  }

  auto left(int i) -> int { return i * 2 + 1; }
  auto right(int i) -> int { return i * 2 + 2; }
  auto mid(int l, int r) -> int { return (l + r) / 2; }

  void set(int i, X x) { dat[i + n - 1] = x; }
  void build()
  {
    for (int k = n - 2; k >= 0; k--)
      dat[k] = fx(dat[2 * k + 1], dat[2 * k + 2]);
  }

  void eval(int k)
  {
    if (lazy[k] == em)
      return;        // 更新するものが無ければ終了
    if (k < n - 1) { // 葉でなければ子に伝搬
      lazy[left(k)] = fm(lazy[left(k)], lazy[k]);
      lazy[right(k)] = fm(lazy[right(k)], lazy[k]);
    }
    dat[k] = fa(dat[k], lazy[k]);
    lazy[k] = em;

    // dat[k] = fa(dat[k], fp(lazy[k], len));
    /*
    len := r-l
    RSQ の時は
    例えば、[1,3) の区間に a を加算したとすれば、その区間の区間和は a*2
    だけ増えるため、その分を節点に加算すれば良い
    */
  }
  void update(int a, int b, M x, int k, int l, int r)
  {
    eval(k);
    if (a <= l && r <= b) { // 完全に内側の時
      lazy[k] = fm(lazy[k], x);
      eval(k);
    }
    else if (a < r && l < b) {                 // 一部区間が被る時
      update(a, b, x, left(k), l, mid(l, r));  // 左の子
      update(a, b, x, right(k), mid(l, r), r); // 右の子
      dat[k] = fx(dat[left(k)], dat[right(k)]);
    }
  }
  void update(int a, int b, M x) { update(a, b, x, 0, 0, n); }
  auto query_sub(int a, int b, int k, int l, int r) -> X
  {
    eval(k);
    if (r <= a || b <= l) { // 完全に外側の時
      return ex;
    }
    else if (a <= l && r <= b) { // 完全に内側の時
      return dat[k];
    }
    else { // 一部区間が被る時
      X vl = query_sub(a, b, left(k), l, mid(l, r));
      X vr = query_sub(a, b, right(k), mid(l, r), r);
      return fx(vl, vr);
    }
  }
  auto query(int a, int b) -> X { return query_sub(a, b, 0, 0, n); }
};

auto main() -> int
{
  int n = 8;
  SegTreeLazy<X, M> rmq(n, fx, fa, fm, ex, em);
}