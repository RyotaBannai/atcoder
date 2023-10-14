def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


def readlines(n: int) -> list[list[int]]:
    return [readline() for _ in range(n)]
    return [readline() for _ in range(n)]


_ = readline()[0]
A = readline()
if len(set(c for c in A)) == 1:
    print("Yes")
else:
    print("No")
