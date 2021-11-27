/*
Takahashi's Secret
*/
#include <iostream>
#include <vector>
using namespace std;

const static int MAX = 100000;
int friends[MAX + 1];

auto main() -> int
{
  int n, cur, cnt = 0;
  cin >> n >> cur;
  for (int i = 0; i <= n; i++) {
    friends[i] = -1;
  }

  int x;
  for (int i = 1; i <= n; i++) {
    cin >> x;
    friends[i] = x;
  }

  int tmp = -1;
  while (cur != -1) {
    cnt++;
    tmp = cur;
    cur = friends[cur];
    friends[tmp] = -1;
  }

  cout << cnt - 1 << endl;
}