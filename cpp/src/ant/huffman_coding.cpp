/*
入力は文字 a b c d e f それぞれの出現頻度
出力はそれぞれの文字をハフマン符号に置き換えた時のビット列
in1:
45 13 12 16 9 5

out1:
0 101 100 111 1101 1100

入力は文字列
in2:
abca

*/
#include <bitset>
#include <iostream>
#include <memory>
#include <queue>
#include <unordered_map>
#include <vector>
using namespace std;

struct Node {
  string c;
  int freq;
  Node *right, *left;
};

auto newNode() -> Node * { return (Node *)malloc(sizeof(Node)); }

void printNode(Node *n)
{
  // cout << n->freq << endl;
  cout << (n->c.length() == 0 ? "node" : "leaf-" + n->c) << ": " << n->freq << endl;

  if (n->left != nullptr)
    printNode(n->left);

  if (n->right != nullptr)
    printNode(n->right);
}

auto findNode(Node *n) -> string
{
  string s = "";
  queue<pair<Node *, string>> q;
  q.push(make_pair(n, ""));
  while (!q.empty()) {
    auto u = q.front();
    q.pop();
    if (u.first->c.length() != 0)
      s += u.second;
    if (u.first->left != nullptr) {
      q.push(make_pair(u.first->left, u.second + "0"));
    }
    if (u.first->right != nullptr) {
      q.push(make_pair(u.first->right, u.second + "1"));
    }
  }

  return s;
}

template <typename T> auto counter(vector<T> V) -> unordered_map<T, int>
{
  unordered_map<string, int> counter;

  for (auto stringval : V) {
    if (counter.find(stringval) == counter.end()) // if key is NOT present already
    {
      counter[stringval] = 1; // initialize the key with value 1
    }
    else {
      counter[stringval]++; // key is already present, increment the value by 1
    }
  }

  // for (auto x : counter)
  //   cout << x.first << "," << x.second << endl;
  return counter;
}

auto main() -> int
{
  vector<string> V;
  string s;
  cin >> s;
  for (auto c : s) {
    string s(1, c);
    V.push_back(s);
  }
  auto cnts = counter(V);

  int N = cnts.size();
  Node *T[N];
  int i = 0;
  for (auto cnt : cnts) {
    int f;
    auto n = newNode();
    n->freq = cnt.second;
    n->c = cnt.first;
    T[i] = n, i++;
  }

  while (N > 1) {
    // cout << "N" << N << endl;
    int m1 = 0, m2 = 1;
    if (T[m1]->freq > T[m2]->freq)
      swap(m1, m2);

    for (int i = 2; i < N; i++) {
      if (T[i]->freq < T[m1]->freq)
        m2 = m1, m1 = i;
      else if (T[i]->freq < T[m2]->freq)
        m2 = i;
    }
    // cout << m1 << "," << m2 << endl;

    auto nn = newNode();
    nn->freq = T[m1]->freq + T[m2]->freq;
    nn->left = T[m2]; // 大きい方が left に来ることで 0 ビットにできる
    nn->right = T[m1];

    if (m1 == N - 1)
      swap(m1, m2);
    T[m1] = nn;
    T[m2] = T[N - 1];
    N--;
  }

  printNode(T[0]);
  // cout << findNode(T[0]).length() << endl;
}