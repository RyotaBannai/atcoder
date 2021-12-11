/*
Neighbors
*/
#include <iostream>
#include <map>
#include <vector>
using namespace std;

/*
1 2 3 の要素と、以下の条件を与えられた時

1 3
2 3
1 2

の時全て 2 つずつ使っているため通ってしまうが、
最後の 1 2 はどう並び替えても隣り合わせにならない.
*/
auto main() -> int
{
  int N, M, a, b;
  cin >> N >> M;
  std::map<int, int> s;

  bool ans = true;
  for (int i = 0; i < M; i++) {
    cin >> a >> b;
    if (s[a] == 1 && s[b] == 1) { //ループするケースの判定できてない
      ans = false;
    }
    s[a]++;
    s[b]++;

    if ((a == 1 || b == 1) && s[1] >= 2) {
      ans = false;
    }
    else if ((a == N || b == N) && s[N] >= 2) {
      ans = false;
    }
    else if (s[a] >= 3 || s[b] >= 3) {
      ans = false;
    }
  }

  cout << (ans ? "Yes" : "No") << endl;
}