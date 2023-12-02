with open("input.txt", "r") as input:
    day1a=0
    data = input.readlines()
    for i, l in enumerate(data):
        a=list("".join(c for c in l if c.isdigit()))
        day1a+=(int(a[0])*10)+int(a[-1])
print(day1a)