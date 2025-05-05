import sys
from game import Game

if (len(sys.argv) > 1):
    in_file = sys.argv[1]
else:
    in_file = input("Enter path to the file with the tags: ")

with open(in_file, 'r') as in_1_file:
    game_values = in_1_file.readlines()
    valid_games = []
    for game_line in game_values:
        g = Game(game_line)
        if g.are_extractions_valid(12,13,14):
            valid_games.append(g.id)
    
    print(f'valid games: {valid_games}')
    print(f'solution: {sum(valid_games)}')