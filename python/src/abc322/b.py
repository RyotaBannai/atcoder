""""""
# import ipdb as pdb
import numpy as np


def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


def readlines(n: int) -> list[list[int]]:
    return [readline() for _ in range(n)]


N, M = readline()
S = input().strip()
T = input().strip()

if T.startswith(S) and T.endswith(S):
    print(0)
elif T.startswith(S):
    print(1)
elif T.endswith(S):
    print(2)
else:
    print(3)
