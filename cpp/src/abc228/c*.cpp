#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

auto main() -> int
{
  int N, K;
  cin >> N >> K;

  vector<int> scores(N), tmp(N);
  for (int i = 0; i < N; i++) {
    scores[i] = 0;
  }

  int pij = 0;
  for (int i = 0; i < N; i++) {
    for (int j = 1; j <= 3; j++) {
      cin >> pij;
      scores[i] += pij;
    }
  }

  tmp = scores;
  // prefer transparent functors 'greater<>' instead of 'greater<int>'
  sort(tmp.begin(), tmp.end(), greater<>());

  int KsTotal = tmp[K - 1];
  for (int x : scores) {
    cout << (KsTotal <= x + 300 ? "Yes" : "No") << endl;
  }
}