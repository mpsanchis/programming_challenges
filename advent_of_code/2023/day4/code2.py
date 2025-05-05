import sys
from lib2 import Scratchcard

if (len(sys.argv) > 1):
    in_file = sys.argv[1]
else:
    in_file = input("Enter path to the file with the tags: ")

scratchcard_pile = {}

def increase_cards(card_num, incr):
    if card_num in scratchcard_pile:
        scratchcard_pile[card_num] += incr
    else:
        scratchcard_pile[card_num] = incr

with open(in_file, 'r') as in_1_file:
    scratchcard_lines = in_1_file.readlines()

    max_line_num = len(scratchcard_lines)
    print(f'number of lines: {max_line_num}')

    for scratchcard_line in scratchcard_lines:
        s = Scratchcard(scratchcard_line)

        increase_cards(s.id, 1)
        copies_card_s = scratchcard_pile[s.id]

        m = s.get_matches()

        for s_won in range(s.id + 1, min(max_line_num+1, s.id + 1 + m)):
            increase_cards(s_won, copies_card_s)
    
    print(f'total cards per id: {scratchcard_pile}')
    print(f'total cards overall: {sum([scratchcard_pile[k] for k in scratchcard_pile.keys()])}')
    


