import re

def load_data(path: str) -> list[str]:
    """Load and split data from file."""
    rows = []
    with open(path, encoding="ascii") as file:
        for row in file:
            rows.append(row.rstrip())
    return rows

data = load_data("14.txt")

