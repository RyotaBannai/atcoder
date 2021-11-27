/*
経路圧縮2
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
  while (q--) {
    int t;
    long long x;
    cin >> t >> x;
    if (t == 1) {
      int pos = x & MASK;
      vector<int> visited{pos};
      while (value[pos] != -1)
        pos = parent[pos], visited.push_back(pos);
      value[pos] = x;
      int new_p = parent[(pos + 1) & MASK];
      /*
      一番初めの u は pos.
      parent[1]==[2]==3 の状態で
      parent[3]==4 になった後に、h==1 の際には
      value[1]!=-1 -> pos==parent[1]==3 -> value[3]!=-1 -> pos==parent[3]==4 -> value[4]==-1
      となる.
      visited には 1,3,4 が入っているため、parent[1]==[3]==[4] に 5 が入る.
      */
      for (int u : visited)
        parent[u] = new_p;
    }
    else {
      cout << value[x & MASK] << '\n';
    }
  }
}
