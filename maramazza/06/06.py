import re

def load_data(path: str) -> list[str]:
    """Load and split data from file."""
    rows = []
    with open(path, encoding="ascii") as file:
        for row in file:
            rows.append(row.rstrip())
    return rows

data = load_data("06.txt")

times = list(map(int, re.findall(r'\d+', data[0])))
dists = list(map(int, re.findall(r'\d+', data[1])))

def ways(time, dist):
    return sum((time - t) * t > dist for t in range(1, time))

result = 1
for t, d in zip(times, dists):
    result *= ways(t, d)
print(result)

t = int(''.join(map(str, times)))
d = int(''.join(map(str, dists)))
print(ways(t, d))