
def cards2type(cards):
    cards_dict = {}
    for c in cards:
        if c in cards_dict:
            cards_dict[c] += 1
        else:
            cards_dict[c] = 1

    cards_values = cards_dict.values()
    if 5 in cards_values:
        return 7
    elif 4 in cards_values:
        return 6
    elif 3 in cards_values:
        if 2 in cards_values:
            return 5
        return 4
    elif 2 in cards_values:
        num_pairs = sum([v==2 for v in cards_values])
        if num_pairs == 2:
            return 3
        return 2
    return 1

def card2card_value(card):
    return 13 - ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'].index(card)

class Hand:
    
    def __init__(self, cards, bid):
        self.cards = cards
        self.type = cards2type(cards)
        self.cardvalues = [card2card_value(self.cards[0]), card2card_value(self.cards[1]), card2card_value(self.cards[2]), card2card_value(self.cards[3]), card2card_value(self.cards[4])]
        self.bid = bid

    def __repr__(self):
        return f'({self.cards}, {self.bid})'

    

        

    
