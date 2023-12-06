import re

def load_data(path: str) -> list[str]:
    """Load and split data from file."""
    rows = []
    with open(path, encoding="ascii") as file:
        for row in file:
            rows.append(row.rstrip())
    return rows

data = load_data("01.txt")

# part 1
total = 0
for line in data:
    digits = re.findall(r'\d', line)
    total += int(digits[0] + digits[-1])
print(total)

# part 2
names = {
    'one': 1, 'two': 2, 'three': 3, 'four': 4, 'five': 5,
    'six': 6, 'seven': 7, 'eight': 8, 'nine': 9,
}

for digit in range(10):
    names[str(digit)] = digit

total = 0
for line in data:
    a = min((line.index(name), value)
        for name, value in names.items() if name in line)
    b = max((line.rindex(name), value)
        for name, value in names.items() if name in line)
    total += int(str(a[1]) + str(b[1]))
print(total)