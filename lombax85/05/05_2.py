from pathlib import Path

def sortFunc(e):
    return e[0]

def lprint(v):
    debug = False
    if debug:
        print(v)

p = Path(__file__).with_name('input.txt')
file = p.open()
file_content = file.read()

elements = file_content.split('\n\n')
seeds = []
elements_parsed = []
conversion_table = {}
all_rules = []

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
            _operation =  _to_start - _from_start
            self.rules_parsed.append((_from_start, _from_end, _operation))
        
        self.rules_parsed.sort(key=sortFunc)
    
    def get_rules_parsed(self):
        return self.rules_parsed
    
    def apply_rules(self, seeds):
        lprint(f'  Applying rules {self.rules_parsed}')
        new_rule_ranges = []

        for seed in seeds:
            seed_from = int(seed[0])
            seed_to = int(seed[1])

            # apply rules to the seed pair. If seed pair starts or ends inside a rule, split the seed pair into new seed pairs
            # example seed pair is (50,60) and rule (55,65,+10), after this we have (50,55), (60,65), (55,60)+10
            # then the rules (operation +-) are applied and the final result is 3 new pairs: (50,55), (60,65), (65,70)
            for rule in self.rules_parsed:
                if rule[0] >= seed_from and rule[1] <= seed_to:
                    new_rule_ranges.append([rule[0], rule[1], rule[2]])
                elif (rule[0] >= seed_from and rule[0] < seed_to) and rule[1] > seed_to:
                     new_rule_ranges.append([rule[0], seed_to, rule[2]])
                elif rule[0] < seed_from and rule[1] >= seed_from:
                    new_rule_ranges.append([seed_from, min(rule[1], seed_to), rule[2]])
                elif rule[0] < seed_from and rule[1] > seed_to:
                    new_rule_ranges.append([seed_from, seed_to, rule[2]])

            # now manage the seed pair part not covered by the rules
            # if it starts before the smallest rule, or it ends after the highest rule
            # add the coresponding new rules
            lowest_rule = self.rules_parsed[0][0]
            highest_rule = self.rules_parsed[-1][1]
            # Add a range before lowest_rule if applicable
            if seed_from < lowest_rule:
                end_point = min(seed_to, lowest_rule - 1)
                new_rule_ranges.append([seed_from, end_point, 0])
            # Add a range after highest_rule if applicable
            if seed_to > highest_rule:
                start_point = max(seed_from, highest_rule + 1)
                new_rule_ranges.append([start_point, seed_to, 0])

        # apply rules and execute operations
        ret = []
        for new_rule in new_rule_ranges:
            ret.append([new_rule[0]+new_rule[2], new_rule[1]+new_rule[2], 0])

        return ret

    def __repr__(self):
        return f'Converter({self.from_to}{str(self.rules_parsed)})'

# parse
for element in elements:
    element_split = element.split(':')
    element_header = element_split[0].replace(' map', '')
    element_content = element_split[1]

    if (element_header == "seeds"):
        seeds = list(filter(lambda x: x != '', element_content.split(' ')))
    else:
        from_to = element_header.split('-to-')
        all_rules.append(Converter(list(filter(lambda x: x != '', element_content.split("\n"))), from_to))

seeds_pairs = []
for i in range(0, len(seeds), 2):
    seeds_pairs.append((seeds[i], seeds[i+1]))

# solution 2
results = []
for pair in seeds_pairs:
    new_pairs = [[int(pair[0]), int(pair[0]) + int(pair[1]) - 1]]
    lprint(f'Working on pair: {new_pairs}')

    for i in range(0,7):
        new_pairs = all_rules[i].apply_rules(new_pairs)
        lprint(f'    Step {i}: {new_pairs}')    

    results.extend(new_pairs)

min_value = min(p[0] for p in results)

print(f'Solution 2: {min_value}')