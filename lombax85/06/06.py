from pathlib import Path
import sys
from functools import reduce
import numpy as np
import math


file_name = Path(__file__).with_name('input.txt')
file_content = file_name.open().read()
print(file_content)

lines = file_content.split('\n')
times = list(filter(lambda x: x != '', lines[0].split('Time:')[1].split(' ')))
distances = list(filter(lambda x: x != '', lines[1].split('Distance:')[1].split(' ')))


# returns the total distance travelled in mm
def race(button_msec, race_msec):
    if button_msec > race_msec:
    
        return 0
    
    speed = button_msec
    remaining_time = race_msec - button_msec
    distance = remaining_time * speed

    return distance


# for solution 2
# I have to find values that satisfies this condition:
# (60808676−x)*x>601116315591300
# or better 
# (60808676−x)*x>y
# I don't remember math from school...so after googling a while, it seems a quadratic inequality
# ax²+bx+c>0
# In our case, y is our target distance 601116315591300
# b is the race time 60808676
# since I'm not interested in solving it using math, I asked chatgpt for help and came with this solution
# for the quadratic inequality

def race2(b, y_target):
    from sympy import symbols, solve, Poly
    x = symbols('x')

    # Coefficients
    a = -1
    c = -y_target

    # Quadratic Equation
    quadratic_eq = Poly(a*x**2 + b*x + c)

    # Solve
    solutions = solve(quadratic_eq, x)

    # Get solutions
    numerical_solutions = [sol.evalf() for sol in solutions]

    v_from = math.floor(numerical_solutions[0])
    v_to = math.floor(numerical_solutions[1])
    
    # get the difference between solutions
    return v_to - v_from


all_distances = []
for i in range(0, len(times)):
    all_distances.append(0)
    race_time = int(times[i])
    distance_to_beat = int(distances[i])

    last_distance = sys.maxsize
    
    button_msec = 1
    while last_distance > 0:
        last_distance = race(button_msec, race_time)
        if (last_distance > distance_to_beat):
            all_distances[i] += 1
        button_msec += 1

result = reduce(lambda x, y: x * y, all_distances)
print(f'Solution 1 is : {result}')

race_time = int(''.join(times))
distance_to_beat = int(''.join(distances))

last_distance = sys.maxsize

print(f'Solution 2 is: {race2(race_time, distance_to_beat)}')








        
        




