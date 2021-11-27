/*
Linear Probing

経路圧縮

https://algo-logic.info/union-find-tree/#toc_id_2
*/
#include <iostream>
#include <vector>
using namespace std;

constexpr int SIZE = 1 << 20;
constexpr int MASK = SIZE - 1;

auto main() -> int
{
  int q;
  cin >> q;
  vector<int> parent(SIZE);
  vector<long long> value(SIZE, -1);
  for (int i = 0; i < SIZE; i++)
    parent[i] = i;
  auto find = [&](auto &&self, int x) -> int {
    /*
    通った全てのノードの親に再起的にルートを指すようにする.
    根でない頂点 := すでに書き換えられた頂点
    根でない頂点から辺を辿っていけば最終的には、根に辿り着いて書き換えられてない番号が分かる
    */
    if (parent[x] == x) {
      return x;
    }
    else {
      return parent[x] = self(self, parent[x]);
    }
  };
  while (q--) {
    int t;
    long long x;
    cin >> t >> x;
    if (t == 1) {
      int i = find(find, x & MASK);
      value[i] = x;
      /*
      今回(i)の頂点も書き換えたため、書き換えていない次のノードを根として指すようにする
      i+1 := 次の頂点が書き換えていなければ、 i+i で、書き換えてあれば、i+1
      が指す書き換えられていない根.
      */
      parent[i] = find(find, (i + 1) & MASK);
    }
    else {
      cout << value[x & MASK] << '\n';
    }
  }
}