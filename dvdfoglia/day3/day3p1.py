def parse_input():
    with open("input.txt", "r") as input:
        d = input.read().splitlines()
        for n, l in enumerate(d):
            d[n]=list(d[n])
    return(d)

def get_numbers(data):
    day3=0
    for row in range(0, len(data)):
        start=False
        number=[0,0,0,0]
        for col in range(0, len(data[row])):
            if data[row][col].isnumeric():
                number[0]=(number[0]*10)+int(data[row][col])
                if not start:
                    number[1]=col
                    number[3]=row
                    start=True
            else:
                start=False
                if number[0] != 0:
                    number[2]=col-1
                    if search_symbols(data, number):
                        day3+=number[0]
                number=[0,0,0,0]
        if start:
            number[2]=col
            if search_symbols(data, number):
                day3+=number[0]
    return(day3)

def search_symbols(data, number):
    startCol=0 if number[1]==0 else number[1]-1
    endCol=number[2]+2 if number[2]<len(data[number[3]])-1 else len(data[number[3]])
    startRow=0 if number[3]==0 else number[3]-1
    endRow=number[3]+2 if number[3]<len(data)-1 else len(data)
    for row in range(startRow,endRow): 
        for col in range(startCol,endCol):
            if not data[row][col].isnumeric() and data[row][col] != ".":
                return True
        
print(get_numbers(parse_input()))