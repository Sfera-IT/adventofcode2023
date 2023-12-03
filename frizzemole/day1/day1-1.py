import re

with open('input.txt') as f:
    sum = 0 
    for line in f:
        find_first_digit = re.search(r'^\D*\d', line)
        find_last_digit = re.search(r'\d\D*$', line)
        number = find_first_digit.group()[-1] + find_last_digit.group()[0]
        sum += int(number)
    print(sum)
f.close()