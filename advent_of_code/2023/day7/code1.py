import sys
from lib1 import Hand

if (len(sys.argv) > 1):
    in_file = sys.argv[1]
else:
    in_file = input("Enter path to the file with the tags: ")


with open(in_file, 'r') as in_1_file:
    input_lines = in_1_file.readlines()

    hands = []
    for input_line in input_lines:
        [cards, bid] = input_line.split(' ')
        hands.append(Hand(cards, int(bid)))
    
    hands.sort(key=lambda h: [h.type, h.cardvalues])

    rank_x_bid = []
    for i,h in enumerate(hands):
        rank_x_bid.append((i+1)*h.bid)
        print(f'hand {h} with rank {i+1}')

    print(f'solution: {sum(rank_x_bid)}')
