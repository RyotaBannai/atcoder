#include <iostream>
#include <string>
#include <vector>
#define lp(i, n) for (int i = 0; i < n; i++)
using namespace std;

struct Node {
  int key;
  Node *next, *prev;
};

auto listSearch(Node *nil, int key) -> Node *
{
  Node *cur = nil->next; // 番兵の次の要素から始める
  while (cur != nil && cur->key != key)
    cur = cur->next;
  return cur;
}

// 表示したい電車の番兵を引数に取る
void printList(Node *nil)
{
  string s = "";
  Node *cur = nil->next;
  int isf = 0;
  while (true) {
    if (cur == nil)
      break;
    if (isf++ > 0)
      s += " ";
    s += to_string(cur->key);
    cur = cur->next;
  }
  cout << isf << " " << s << endl;
}

auto newNil() -> Node *
{
  Node *nil = (Node *)malloc(sizeof(Node));
  nil->next = nil;
  nil->prev = nil;
  return nil;
}

auto newTrain(int key) -> Node *
{
  Node *x = (Node *)malloc(sizeof(Node));
  x->key = key;

  Node *nil = (Node *)malloc(sizeof(Node));
  nil->next = x;
  nil->prev = x;

  x->next = nil;
  x->prev = nil;

  return nil;
}

auto detach(Node *head, Node *fst) -> Node *
{
  Node *nil = newNil();
  nil->next = fst->next;
  nil->next->prev = nil;
  nil->prev = head->prev;
  head->prev->next = nil; // 新しい電車の先頭に切り替える

  fst->next = head; // 元々の電車の先頭
  head->prev = fst; // 元々の電車の先頭の前

  cout << "detach" << endl;

  return nil;
}

void attach(Node *fst, Node *snd)
{
  /*
  fst は車両の末尾、snd は車両の先頭
  snd の前後は完全に置き換わるため、snd の番兵は解放
  */
  free(snd->prev);
  snd->next = fst->next;
  fst->next = snd;
  snd->prev = fst;
  snd->next->prev = snd;

  cout << "attach" << endl;
}

int N, M;
vector<Node *> trains;

using P = pair<int, Node *>;
auto findTrain(int x) -> P
{
  Node *t;
  lp(i, N)
  {
    t = listSearch(trains[i], x);
    if (t != trains[i]) { // もし電車が見つかったら
      return make_pair(i, t);
    }
  }

  return make_pair(-100, t);
}

auto main() -> int
{
  cin >> N >> M;
  trains.reserve(N);
  lp(i, N) trains[i] = newTrain(i + 1);
  lp(i, M)
  {
    int q, x, y;
    cin >> q;
    if (q == 3) {
      cin >> x;
      P t = findTrain(x);
      printList(trains[t.first]);
    }
    else {
      cin >> x >> y;
      if (q == 1) {
        P t1 = findTrain(x);
        P t2 = findTrain(y);
        attach(t1.second, t2.second);
        remove(trains.begin(), trains.end(), trains[t2.first]);
      }
      else if (q == 2) {
        P t = findTrain(x);
        Node *nil = detach(trains[t.first], t.second);
        trains.push_back(nil);
      }
    }
  }
}