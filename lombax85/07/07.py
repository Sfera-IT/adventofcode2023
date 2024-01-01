from pathlib import Path
import sys
from functools import reduce
import numpy as np
import math


file_name = Path(__file__).with_name('input.txt')
file_content = file_name.open().read()

lines = file_content.split('\n')

cardValues = {
    "A": 14,
    "K": 13,
    "Q": 12,
    "J": 11,
    "T": 10,
    "9": 9,
    "8": 8,
    "7": 7,
    "6": 6,
    "5": 5,
    "4": 4,
    "3": 3,
    "2": 2
}

from enum import Enum
class HandValue(Enum):
    FiveOfAKind = 6
    FourOfAKind = 5
    FullHouse = 4
    ThreeOfAKind = 3
    TwoPair = 2
    OnePair = 1
    HighCard = 0

handValues = {
    "5" : HandValue.FiveOfAKind,
    "41" : HandValue.FourOfAKind,
    "32" : HandValue.FullHouse,
    "311" : HandValue.ThreeOfAKind,
    "221" : HandValue.TwoPair,
    "2111" : HandValue.OnePair,
    "11111" : HandValue.HighCard
}


class Hand:
    def __init__(self, cards, bid) -> None:
        self.textCards = cards
        self.bid = int(bid)

        # decompose cards
        self.cards = self.calcSingleCards()
        # get the hand type
        self.handType = self.calcHandType()

    def calcHandType(self):
        values = sorted(self.cards.values(), reverse=True)
        values = ''.join(map(str, values))

        if values not in handValues:
            raise Exception('this should not happen')
        return handValues[values]

    def calcSingleCards(self):
        cards = list(self.textCards)
        dict = {}
        for card in cards:
            if not card in dict:
                dict[card] = 1
            else:
                dict[card] += 1
        return dict

    def __repr__(self):
        return f'Hand({self.handType} - {self.textCards})'

allCards = []
for line in lines:
    elements = line.split(" ")
    cards = elements[0]
    bid = elements[1]
    allCards.append(Hand(cards, bid))
    


from functools import cmp_to_key

def custom_comparison_function(item1, item2):
    # Implement your comparison logic here
    # Return -1 if item1 < item2, 0 if item1 == item2, 1 if item1 > item2
    if item1.handType.value < item2.handType.value:
        return -1
    elif item1.handType.value > item2.handType.value:
        return 1
    else:
        for i in range(0,5):
            v1 = cardValues[item1.textCards[i]]
            v2 = cardValues[item2.textCards[i]]
            if v1 < v2:
                return -1
            elif v1 > v2:
                return 1                
        
        raise Exception('Dov\'e Bugo')
        return 0

allCards.sort(key=cmp_to_key(custom_comparison_function))

rank = 1
tot = 0
for card in allCards:
    tot += card.bid*rank
    rank += 1


print(f'Solution 1 is: {tot}')


        

