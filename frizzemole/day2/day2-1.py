import re

REDS = 12
GREEN = 13
BLUES = 14

with open('input.txt') as f:
    game_ids = 0
    for line in f:
        line = line.strip()
        line_split = line.split(':')
        game_id = re.search(r'\d*$', line_split[0]).group()
        draws = line_split[1].split(';')
        possible = 1
        for draw in draws:
            red, green, blue = 0, 0, 0
            red_match = re.search(r'(\d*) red', draw)
            if red_match:
                red = int(red_match.group(1))
            green_match = re.search(r'(\d*) green', draw)
            if green_match:
                green = int(green_match.group(1))

            blue_match = re.search(r'(\d*) blue', draw)
            if blue_match:
                blue = int(blue_match.group(1))

            if red > REDS or green > GREEN or blue > BLUES:
                possible = 0
                break
        if possible == 1:
            game_ids += int(game_id)
    print(game_ids)