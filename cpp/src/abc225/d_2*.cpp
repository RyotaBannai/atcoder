/* @cpg_dirspec d
Play Train

https://atcoder.jp/contests/abc225/tasks/abc225_d

AC

*/
#include <iostream>
#include <map>
#include <string>
#include <vector>
using namespace std;

auto main() -> int
{
  int N, M;
  cin >> N >> M;
  /**
   * p は key に子、value に親
   * c は key に親、value に子
   */
  map<int, int> p, c; // <child, parent>, <parent, child>

  for (int i = 0; i < M; ++i) {
    int q, x, y;
    cin >> q;
    if (q == 3) {
      cin >> x;
      int parent = x;
      while (true) {
        if (!p[parent]) {
          break;
        }
        else {
          parent = p[parent];
        }
      }

      int child = parent; // parent == 連結成分の root
      vector<int> ans{child};
      while (true) {
        if (!c[child]) { // 子を持たない
          break;
        }
        else {
          child = c[child];
          ans.push_back(child);
        }
      }

      // 結果を表示
      cout << ans.size() << " ";
      for (auto x : ans) {
        cout << x << " ";
      }
      cout << endl;
    }
    else {
      cin >> x >> y;
      if (q == 1) { // 連結
        p[y] = x;
        c[x] = y;
      }
      else if (q == 2) { // 分解
        p[y] = 0;
        c[x] = 0;
      }
    }
  }
}