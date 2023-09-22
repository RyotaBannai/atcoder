"""https://atcoder.jp/contests/abc047/tasks/abc047_b.

#numpy
"""
# import ipdb as pdb
import numpy as np


def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


def readlines(n: int) -> list[list[int]]:
    return [readline() for _ in range(n)]


W, H, N = readline()
dat = readlines(N)

g = np.ones((H, W))  # row x column で指定

for x, y, a in dat:
    match a:
        case 1:
            g[:, :x] = 0
        case 2:
            g[:, x:] = 0
        case 3:
            g[:y, :] = 0
        case 4:
            g[y:, :] = 0

print(g.sum(dtype=int))
