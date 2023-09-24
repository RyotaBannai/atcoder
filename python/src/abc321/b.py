def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


N, X = readline()
N -= 1
As = sorted(readline())


t = sum(As[: N - 1])
if t >= X:
    print(0)
    exit(0)

t = sum(As[1:])
if t < X:
    print(-1)
    exit(0)

t = sum(As[1 : N - 1])
print(X - t)
