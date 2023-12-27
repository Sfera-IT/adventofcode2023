from pathlib import Path

p = Path(__file__).with_name('input.txt')
file_content = p.open().read()

elements = file_content.split('\n\n')

seeds = []
elements_parsed = []
conversion_table = {}

class Converter():
    def __init__(self, rules, from_to):
        self.rules = rules
        self.from_to = from_to
        self.rules_parsed = []

        for rule in rules:
            rule = rule.split(' ')
            _delta = int(rule[2])
            _from_start = int(rule[1])
            _from_end = _from_start + _delta - 1
            _to_start = int(rule[0])
            _to_end = _to_start + _delta - 1
            _operation =  _to_start - _from_start
            self.rules_parsed.append((_from_start, _from_end, _to_start, _to_end, _operation))

    def apply_rules(self, seed):
        seed = int(seed)
        for rule in self.rules_parsed:
            if seed >= rule[0] and seed <= rule[1]:
                seed = seed + rule[4]
                return seed
        return seed
    
    def apply_rules2(self, seed_from, seed_to):
        seed_from = int(seed_from)
        seed_to = int(seed_to)

        for rule in self.rules_parsed:
            if seed >= rule[0] and seed <= rule[1]:
                seed = seed + rule[4]
                return seed
        return seed

    def __repr__(self):
        return f'Converter({self.from_to}{str(self.rules_parsed)})'


for element in elements:
    element_split = element.split(':')
    element_header = element_split[0].replace(' map', '')
    element_content = element_split[1]

    if (element_header == "seeds"):
        seeds = list(filter(lambda x: x != '', element_content.split(' ')))
    else:
        from_to = element_header.split('-to-')
        _from = from_to[0]
        _to = from_to[1]
        if _from not in conversion_table:
            conversion_table[_from] = {}
        conversion_table[_from][_to] = Converter(list(filter(lambda x: x != '', element_content.split("\n"))), from_to)


lowest_location = 100000000000000000
for seed in seeds:
    converter = conversion_table['seed']['soil']
    seed = converter.apply_rules(seed)

    converter = conversion_table['soil']['fertilizer']
    seed = converter.apply_rules(seed)

    converter = conversion_table['fertilizer']['water']
    seed = converter.apply_rules(seed)

    converter = conversion_table['water']['light']
    seed = converter.apply_rules(seed)

    converter = conversion_table['light']['temperature']
    seed = converter.apply_rules(seed)

    converter = conversion_table['temperature']['humidity']
    seed = converter.apply_rules(seed)

    converter = conversion_table['humidity']['location']
    seed = converter.apply_rules(seed)

    if seed < lowest_location:
        lowest_location = seed

print("Solution 1:", lowest_location)