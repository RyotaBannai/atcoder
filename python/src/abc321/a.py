def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


def readlines(n: int) -> list[list[int]]:
    return [readline() for _ in range(n)]


N = str(readline()[0])
for i in range(len(N) - 1):
    if int(N[i]) <= int(N[i + 1]):
        print("No")
        exit(0)

print("Yes")
