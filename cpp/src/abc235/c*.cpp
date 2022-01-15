/* @cpg_dirspec c
The Kth Time Query
*/
#include <iostream>

#include <map>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

auto main() -> int
{
  int N, Q;
  cin >> N >> Q;
  map<int, vector<int>, std::less<>> A;
  for (int i = 0; i < N; i++) {
    int a;
    cin >> a;
    if (A.count(a)) // already exist
      A[a].push_back(i + 1);
    else
      A[a] = vector<int>{i + 1};
  }

  while (Q--) {
    int x, k;
    cin >> x >> k;
    if (!A.count(x)) { // doesn't exist
      cout << (-1) << endl;
      continue;
    }

    if (k <= A[x].size()) // size is too small
      cout << A[x][k - 1] << endl;
    else
      cout << (-1) << endl;
  }
}