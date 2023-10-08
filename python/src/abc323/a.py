"""."""
# import ipdb as pdb
# import numpy as np


def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


def readlines(n: int) -> list[list[int]]:
    return [readline() for _ in range(n)]


S = input().strip()
ans = True
for i in range(16):
    if i % 2 == 1:
        ans &= S[i] == "0"


if ans:
    print("Yes")
else:
    print("No")
