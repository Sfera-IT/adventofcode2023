max_c = {'red': 0, 'green': 0, 'blue': 0}
day2=0

with open("input.txt", "r") as input:
    d = input.read().splitlines()
    for l in d:
        l=l.replace("Game ","").split(sep=":")
        sets=l[1].split(sep=";")
        for set in sets:
            cubes=set.split(sep=",")
            for c in cubes:
                c=c.strip().split(sep=" ")
                for k,v in max_c.items():
                    if k == c[1] and int(c[0]) > v:
                        max_c[k]=int(c[0])
        day2+=(max_c["red"]*max_c["green"]*max_c["blue"])
        max_c = {'red': 0, 'green': 0, 'blue': 0}
print(day2)