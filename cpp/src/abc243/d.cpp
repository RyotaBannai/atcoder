/* @cpg_dirspec d
Moves on Binary Tree

https://atcoder.jp/contests/abc243/tasks/abc243_d

 */
#include <cstdio>
#include <iomanip> // for std::setprecision()
#include <iostream>
#include <string>
#include <vector>
using namespace std;
using ll = long long;

struct Node {
  int key;
  Node *parent, *left, *right;
};

Node *root, *NIL;

template <typename T> auto in() -> T
{
  T inp;
  cin >> inp;
  return inp;
}

auto allocNode() -> Node * { return (Node *)malloc(sizeof(Node)); }

void insert(int v)
{
  Node *y = NIL;
  Node *x = root;
  Node *z = allocNode(); // create new Node
  z->key = v;
  z->left = NIL;
  z->right = NIL;

  while (x != NIL) {
    y = x;
    x = z->key < x->key ? x->left : x->right;
  }

  z->parent = y;
  if (y == NIL)
    root = z;
  else {
    if (z->key < y->key)
      y->left = z;
    else
      y->right = z;
  }
}

auto find(Node *u, int v) -> Node *
{
  while (u != NIL && u->key != v)
    u = v < u->key ? u->left : u->right;
  return u;
}

auto main() -> int
{
  ll n, x;
  cin >> n >> x;
  string S;
  cin >> S;
  ll max = (1 << n) - 1;
  for (int i = 1; i < max; ++i) {
    insert(i);
  }

  // URL
  Node *u = find(root, x);

  // cout << S << endl;
  // cout << u->key << endl;

  for (char c : S) {
    if (u->right != NIL && c == 'R') {

      u = u->right;
    }

    if (u->left != NIL && c == 'L') {
      u = u->left;
    }

    if (u->parent != NIL && c == 'U') {
      // cout << u->right->key << endl;
      u = u->parent;
    }
  }

  // cout << u->key << endl;
}
