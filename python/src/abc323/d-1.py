"""https://atcoder.jp/contests/abc323/tasks/abc323_d.

X を2つ足して2X にする操作を繰り返すことは、
他のYを2つ足して2Y にすることを繰り返すことと完全に独立している.
そうすると2X をheap に入れたあと他の合成数2Y を待つ必要がない.
-> heap もコストがかかるがそれが不要になって、最小を取り出すだけよく、
2X をpushして次に取り出す順序を決めるように管理するコストも不要.
-> 最小取り出す: 入力値がすでに最小値から与えらえる

heap してもC++くらい高速なら通りそう?
"""
from collections import defaultdict as dd

# import ipdb as pdb


def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


def readlines(n: int) -> list[list[int]]:
    return [readline() for _ in range(n)]


N = readline()[0]
size = dd(int)
for _ in range(N):
    s, c = readline()
    size[s] += c
    while size[s] >= 2:
        ns = s * 2
        size[ns] += size[s] // 2
        size[s] %= 2
        s = ns

# pdb.set_trace()
ans = sum(v for _, v in size.items())
print(ans)
