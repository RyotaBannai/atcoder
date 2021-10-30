/*
Permutation
n=3 の時:
0 1 2
0 2 1
1 0 2
1 2 0
2 0 1
2 1 0

*/
#include <algorithm>
#include <iostream>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
#define lps(i, j, n) for (int i = j; i < n; i++)
using namespace std;

const static int MAX_N = 100000; // 桁数
bool used[MAX_N];
int perm[MAX_N];

void permutation1(int pos, int n)
{
  if (pos == n) {
    /*
    perm に対する処理
    */

    // lp(i, n) { cout << perm[i] << " "; }
    // cout << endl;
    return;
  }

  lp(i, n)
  {
    if (!used[i]) {
      perm[pos] = i;
      used[i] = true;
      permutation1(pos + 1, n);
      used[i] = false;
    }
  }
}

int perm2[MAX_N];

/*
重複があっても全ての順列を生成
next_permutation は辞書順で次の順列を生成
*/
void permutation2(int n)
{
  lp(i, n) perm2[i] = i;
  do {
    /*
    perm に対する処理
    */

    // lp(i, n) { cout << perm[i] << " "; }
    // cout << endl;
  } while (next_permutation(perm2, perm2 + n));
  // 全ての順列を生成し終わった時に next_permutation は false を返す.
}

auto main() -> int { permutation2(3); }