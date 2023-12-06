from collections import defaultdict
import re

def load_data(path: str) -> list[str]:
    """Load and split data from file."""
    rows = []
    with open(path, encoding="ascii") as file:
        for row in file:
            rows.append(row.rstrip())
    return rows

data = load_data("02.txt")

part1 = part2 = 0
limit = dict(red=12, green=13, blue=14)
for line in data:
    a, b = line.split(':')
    fewest = defaultdict(int)
    ok = True
    for r in b.split(';'):
        for n, c in re.findall(r'(\d+) (red|green|blue)', r):
            fewest[c] = max(fewest[c], int(n))
            ok = ok and int(n) <= limit[c]
    part1 += int(a.split()[-1]) if ok else 0
    part2 += fewest['red'] * fewest['green'] * fewest['blue']

print(part1)
print(part2)