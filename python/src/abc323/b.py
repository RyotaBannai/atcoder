"""."""
# import ipdb as pdb
# import numpy as np
from collections import defaultdict as dd


def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


def readlines(n: int) -> list[list[int]]:
    return [readline() for _ in range(n)]


N = readline()[0]
dat = [input().strip() for _ in range(N)]

res = dd(int)
for i in range(N):
    res[i]

for i in range(N):
    S = dat[i]
    for j in range(N):
        if S[j] == "-":
            continue
        if S[j] == "o":
            res[i] += 1
        else:
            res[j] += 1


def cmp(a, b):
    if a[0] == b[0]:
        return a[1] < b[1]
    else:
        return a[0] < b[0]


ans = sorted([(v, -k) for k, v in res.items()], reverse=True)
print(" ".join([str(abs(k) + 1) for v, k in ans]))
