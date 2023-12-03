numbers = {'one': 1, 'two': 2, 'three': 3, 'four': 4, 'five': 5, 'six': 6, 'seven': 7, 'eight': 8, 'nine': 9}

def convert(s):
    for w, n in numbers.items():
        if w in s:
            s = s.replace(w, w+str(n)+w)
    return s

with open("input.txt", "r") as input:
    day1a=0
    data = input.readlines()
    for i, l in enumerate(data):
        l=convert(l)
        a=list("".join(c for c in l if c.isdigit()))
        day1a+=(int(a[0])*10)+int(a[-1])
print(day1a)