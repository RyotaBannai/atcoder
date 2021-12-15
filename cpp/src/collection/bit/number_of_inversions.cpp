/*
The number of inversions with
Binary Indexed Tree(BIT)

References:
https://coonevo.hatenablog.com/entry/2020/03/19/174849#%E9%AB%98%E9%80%9F%E5%8C%96ONlogN


数列 c の中に、bi 以下のものがいくつあるかを高速に求めたい（つまり、数列 c の prefix の和がほしい）
-> Range Sum Query
Range Sum Query は セグ木、BIT（Binary IndexedTree）などのデータ構造で解くことができる
c を BIT に埋め込んで、prefix の和を求められれば、O(NlogN)で解くことができる
*/

#include <iostream>
#include <vector>
#define print(v)                                                                                   \
  {                                                                                                \
    cerr << #v << ": [ ";                                                                          \
    for (auto _ : v)                                                                               \
      cerr << _ << ", ";                                                                           \
    cerr << "]" << endl;                                                                           \
  }
using namespace std;
using ll = long long;

template <typename T> struct BIT {
  int n;
  vector<T> bit;
  BIT(int leafs, int defa = 0) : n(leafs + 1), bit(n, defa) {}

  // i is 1-indexed
  void add(int i, T x)
  {
    for (int idx = i; idx < n; idx += (idx & -idx)) {
      bit[idx] += x;
    }
  }
  // 1-indexed sum of prefix [1,a)
  auto sum(int i) -> T
  {
    T s(0);
    for (int idx = i; idx > 0; idx -= (idx & -idx)) {
      s += bit[idx];
    }
    return s;
  }
  // 1-index sum of range [l,r)
  // sum で引数を r or r-1 とするかは r を含めるか含めないかに依存
  auto sum(int l, int r) -> T { return sum(r - 1) - sum(l - 1); }

  // 0-indexed add （利用元は 0 index を使っているため、呼び出された時に 1 index へ変換）
  void add0(int i, T x) { add(i + 1, x); }
  // 0-indexed sum
  auto sum0(int i) -> T { return sum(i + 1); }
  // 0-indexed sum of range
  auto sum0(int l, int r) -> T { return sum0(r - 1) - sum0(l - 1); }

  void debug() { print(bit); }
  inline auto operator[](int i) -> T { return bit[i]; }
};

auto main() -> int
{
  int N;
  cin >> N;
  vector<ll> a(N), b(N);    // a:= 入力順の数列
  vector<pair<ll, int>> ap; // 数列と入力された時の順番
  for (int i = 0; i < N; i++) {
    cin >> a[i];
    ap.emplace_back(a[i], i);
  }

  // 入力された数列の値でソート = ソートされた時にどれくらい移動するかがわかる.
  sort(ap.begin(), ap.end());
  for (int i = 0; i < N; i++) {
    /*
    b:= i 番目に入力された要素が、ソートされた時に何番目に来るかを管理

    b[入力された位置(=i)] = ソートされた位置
    （i=0 から取り出した時に a に入力された数列の要素がソートされた時にどの位置に来るかが分かる）
    */
    b[ap[i].second] = i;
  }
  BIT<ll> c(N);
  ll ans = 0;
  for (int i = 0; i < N; i++) {
    ans += i - c.sum0(b[i]);
    c.add0(b[i], 1);
  }

  /*
  Reasonings
  3 1 2 の配列があるときの処理を考える.

  i - c.sum0(b[i]);
  では、
  2 - 1 となる
  2 は一番後ろになるが、１の後に配置で良いため自分より前の要素数を引く.

  2:= 自分があるべき位置
  1:= 自分を配置するときに前にある要素数

  2 の前に入っていくる要素が後から出てくる可能性があるが、その時はその要素を基に計算する際に１つ分
  ans に足せば良い（１回の反転で 2 要素の位置が変わるため２回操作しない）
  */

  cout << ans << endl;
}