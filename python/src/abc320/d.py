""".

defaultdict 
[] はassignment
get はreference 
として使い分ける

[] でreference すると値が作られてしまう.
"""

from collections import defaultdict as dd
from collections import deque
from typing import Literal


def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


def readlines(n: int) -> list[list[int]]:
    return [readline() for _ in range(n)]


N, M = readline()

if M == 0:
    print("0 0")
    for i in range(1, N):
        print("undecidable")
    exit(0)

dat = readlines(M)
adj: dd[int, list[tuple[int, int, int]]] = dd(list)
for a, b, x, y in dat:
    adj[a].append((b, x, y))
    adj[b].append((a, -x, -y))

pos: dd[int, tuple[int, int]] = dd(None)
undecisible: dd[int, Literal[True]] = dd(lambda: True)

q: deque[tuple[int, int, int]] = deque()
q.append((1, 0, 0))
while len(q) > 0:
    u, x, y = q.pop()
    if pos.get(u) is not None:
        if pos.get(u) != (x, y):
            undecisible[u]

        continue
    pos[u] = (x, y)
    for w, dx, dy in adj[u]:
        q.append((w, x + dx, y + dy))


for i in range(N):
    if undecisible.get(i + 1):
        print("undecidable")
    else:
        v = pos.get(i + 1)
        if not v:
            print("undecidable")
        else:
            print(*v)
