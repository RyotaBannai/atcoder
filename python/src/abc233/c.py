"""https://atcoder.jp/contests/abc233/tasks/abc233_c.

選ぶ数が複数あるなら再帰

#再帰 #袋数 #数え上げ #組合せ
"""


def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


def readlines(n: int) -> list[list[int]]:
    return [readline() for _ in range(n)]


N, X = readline()
As = readlines(n=N)

count = 0


def rec(i: int, parent: int):
    global count
    if i == N or parent > X:
        return
    for j in As[i][1:]:
        m = parent * j
        if m == X and i == N - 1:  # 各袋から１つずつ取り出す.
            count += 1
        rec(i=i + 1, parent=m)


rec(i=0, parent=1)

print(count)
