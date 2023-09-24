def readline() -> list[int]:
    return list(map(int, input().strip().split(" ")))


N = readline()[0]
k = 0
for d, s in enumerate(str(N)[1:-1][::-1]):
    k += int(s) * 10**d


def calc(d: int, head: int, tail: int) -> tuple[int, bool]:
    if d == 1:
        return 1 if head <= N else 0, head > N
    elif d == 2:
        a = head * 10
        a += tail
        return 1 if a <= N else 0, a > N

    d -= 1
    base = head * 10**d
    base += tail
    tmp = 0
    for i in range(0, d - 1):
        tmp += 9 * 10**i
        mi = min(base + tmp * 10, N)
        if mi == N:
            return max(k - 1, 0), True
    return tmp, False


count = 0


for i in range(1, 10):
    for j in range(1, 10):
        for a in range(1, 7):
            num_a, end_a = calc(a, i, j)
            for b in range(1, 7):
                num_b, end_b = calc(b, j, i)
                count += num_a * num_b
                if end_b:
                    break

            if end_a:
                break

print(count)
