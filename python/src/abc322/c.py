""""""
# import ipdb as pdb
import numpy as np


def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


def readlines(n: int) -> list[list[int]]:
    return [readline() for _ in range(n)]


N, M = readline()
A = readline()

ans = []
prev = 0
for i in range(M):
    a = A[i]
    i += 1
    l = list(range(a - prev))
    # print(l)
    l.reverse()
    ans += l
    prev = a

for x in ans:
    print(x)
