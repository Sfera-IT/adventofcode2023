import re

with open('input.txt') as f:
    games_power = 0
    for line in f:
        line = line.strip()
        line_split = line.split(':')
        game_id = re.search(r'\d*$', line_split[0]).group()
        draws = line_split[1].split(';')
        reds = []
        greens = []
        blues = []
        for draw in draws:
            red_match = re.search(r'(\d*) red', draw)
            if red_match:
                reds.append(int(red_match.group(1)))
            green_match = re.search(r'(\d*) green', draw)
            if green_match:
                greens.append(int(green_match.group(1)))

            blue_match = re.search(r'(\d*) blue', draw)
            if blue_match:
                blues.append(int(blue_match.group(1)))
        
        games_power += max(reds) * max(greens) * max(blues)
    
    print(games_power)
        
        