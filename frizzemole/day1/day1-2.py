import re

numbers = {
    '1': '1',
    '2': '2',
    '3': '3',
    '4': '4',
    '5' : '5',
    '6' : '6',
    '7' : '7',
    '8' : '8',
    '9' : '9',
    'one' : '1',
    'two' : '2',
    'three' : '3',
    'four' : '4',
    'five' : '5',
    'six' : '6',
    'seven' : '7',
    'eight' : '8',
    'nine' : '9'
}

def sort_tuples(sub_li):
    sub_li.sort(key=lambda x: x[1])
    return sub_li

with open('input.txt') as f:
    sum = 0
    for line in f:
        founds = []
        for key in numbers.keys():
            for match in re.finditer(key, line):
                founds.append([numbers[key], match.start()])
        if len(founds) == 1:
            number = founds[0][0] + founds[0][0]
        else:
            founds = sort_tuples(founds)
            first_digit = founds[0][0]
            last_digit = founds[-1][0]
            number = first_digit + last_digit

        sum += int(number)
    print(sum)
f.close()