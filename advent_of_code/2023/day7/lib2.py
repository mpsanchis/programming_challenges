def cards2dict(cards):
    cards_dict = {}
    for c in cards:
        if c in cards_dict:
            cards_dict[c] += 1
        else:
            cards_dict[c] = 1

    return cards_dict


def cards2type(cards):
    cards_dict = cards2dict(cards)

    cards_values = cards_dict.values()
    if 5 in cards_values: # Five of a kind
        return 7
    elif 4 in cards_values: # Four of a kind
        return 6
    elif 3 in cards_values:
        if 2 in cards_values: # Full house
            return 5
        return 4 # Three of a kind
    elif 2 in cards_values:
        num_pairs = sum([v==2 for v in cards_values])
        if num_pairs == 2: # Two pairs
            return 3
        return 2 # Pair
    return 1 # High card

def typenum2str(typenum):
    
    kinds = ['5 of a kind','4 of a kind','full house','3 of a kind','2 pairs','pair','high card']

    return kinds[7-typenum]


def card2card_value(card):
    return 13 - ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'].index(card)

def most_common_cards(cards, omit_joker):
    card_dict = cards2dict(cards)
    if omit_joker:
        card_dict = {x:card_dict[x] for x in [k for k in card_dict.keys() if k!='J']}

    max_dupl = max(card_dict.values())

    most_common_cards = []
    for c in card_dict.keys():
        if (card_dict[c] == max_dupl):
            most_common_cards.append(c)

    return most_common_cards

def highest_most_common_card(cards, omit_joker):
    mcc = most_common_cards(cards, omit_joker)
    
    return max(mcc, key=lambda c: card2card_value(c))

def transform_cards_with_joker(cards):
    if (not 'J' in cards):
        return cards

    num_jokers = sum([c=='J' for c in cards])

    if num_jokers == 5:
        return 'AAAAA'

    hmcc = highest_most_common_card(cards, omit_joker=True)

    transformed_cards = ''
    for c in cards:
        if c == 'J':
            transformed_cards += hmcc
        else:
            transformed_cards += c

    return transformed_cards


class Hand:
    
    def __init__(self, cards, bid):
        self.cards = cards
        self.transformed_cards = transform_cards_with_joker(cards)
        self.type = cards2type(self.transformed_cards)
        self.cardvalues = [card2card_value(self.cards[0]), card2card_value(self.cards[1]), card2card_value(self.cards[2]), card2card_value(self.cards[3]), card2card_value(self.cards[4])]
        self.bid = bid

    def __repr__(self):
        return f'{typenum2str(self.type)} ({self.cards} ({self.transformed_cards}), {self.bid})'

    

        

    
