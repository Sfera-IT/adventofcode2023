from collections import *
import re

def load_data(path: str) -> list[str]:
    """Load and split data from file."""
    rows = []
    with open(path, encoding="ascii") as file:
        for row in file:
            rows.append(row.rstrip())
    return rows

data = load_data("04.txt")

total = 0
counts = defaultdict(int)
for line in data:
    nums = re.findall(r'\d+|\|', line)
    card = int(nums.pop(0))
    i = nums.index('|')
    count = len(set(nums[:i]) & set(nums[i:]))
    counts[card] += 1
    if count:
        total += 2 ** (count - 1)
        for i in range(card + 1, card + 1 + count):
            counts[i] += counts[card]

print(total)
print(sum(counts.values()))