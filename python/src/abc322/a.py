""""""
# import ipdb as pdb
import numpy as np


def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


def readlines(n: int) -> list[list[int]]:
    return [readline() for _ in range(n)]


N = readline()[0]
S = input().strip()

for i in range(N - 2):
    if S[i : i + 3] == "ABC":
        print(i + 1)
        exit(0)

print(-1)
