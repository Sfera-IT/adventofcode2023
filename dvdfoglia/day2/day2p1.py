max_c = {'red': 12, 'green': 13, 'blue': 14}
day2=0

with open("input.txt", "r") as input:
    d = input.read().splitlines()
    for l in d:
        l=l.replace("Game ","").split(sep=":")
        sets=l[1].split(sep=";")
        possible=True
        for set in sets:
            cubes=set.split(sep=",")
            for c in cubes:
                c=c.strip().split(sep=" ")
                for k,v in max_c.items():
                    if k in c and int(c[0]) > v:
                        possible=False
        if possible:
            day2+=int(l[0])
print(day2)