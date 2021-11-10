/*
入力は文字 a b c d e f それぞれの出現頻度
出力はそれぞれの文字をハフマン符号に置き換えた時のビット列

in:
45 13 12 16 9 5

out:
0 101 100 111 1101 1100
*/
#include <iostream>
#include <vector>
using namespace std;

struct Node {
  char c;
  int freq;
  Node *right, *left;
};

auto newNode() -> Node * { return (Node *)malloc(sizeof(Node)); }

auto main() -> int
{
  vector<char> alpha = {'a', 'b', 'c', 'd', 'e', 'f'};
  int N = alpha.size();
  Node *T[N];
  for (int i = 0; i < N; i++) {
    int f;
    auto n = newNode();
    cin >> f;
    n->freq = f;
    n->c = alpha[i];
    T[i] = n;
  }

  while (N > 1) {
    int m1 = 0, m2 = 1;
    if (T[m1]->freq > T[m2]->freq)
      swap(m1, m2);

    for (int i = 2; i < N; i++) {
      if (T[i]->freq < T[m1]->freq)
        m2 = m1, m1 = i;
      else if (T[i]->freq < T[m2]->freq)
        m2 = i;
    }

    auto nn = newNode();
    nn->freq = T[m1]->freq + T[m2]->freq;

    if (m1 == N - 1)
      swap(m1, m2);
    T[m1] = nn;
    T[m2] = T[N - 1];
    N--;
  }
}