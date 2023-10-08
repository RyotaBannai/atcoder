"""."""
# import numpy as np
from collections import defaultdict as dd

# import ipdb as pdb


def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


def readlines(n: int) -> list[list[int]]:
    return [readline() for _ in range(n)]


N, M = readline()
As = readline()
dat = [input().strip() for _ in range(N)]


total: dd[int, int] = dd(int)
SolvedBy: dd[int, set[int]] = dd(set)
for i in range(N):
    tmp = i
    for j in range(M):
        if dat[i][j] == "o":
            tmp += As[j]
            SolvedBy[j].add(i)

    total[i] = tmp

reduced: dd[int, int] = dd(int)
for _, v in total.items():
    reduced[v] += 1

ma = max(total.values())

AsSorted = sorted([(v, k) for k, v in enumerate(As)], reverse=True)

# pdb.set_trace()

for i in range(N):
    v = total.get(i)
    if v is None:
        raise Exception("not expected")

    ns = reduced[v]
    if ns is None:
        raise ("ns should exist")
    if ns == 1 and ma == v:
        print(0)
        continue

    count = 0
    for s, k in AsSorted:
        if i in SolvedBy[k]:
            continue
        v += s
        count += 1
        if v > ma:
            print(count)
            break
