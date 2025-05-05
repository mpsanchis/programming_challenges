import sys
from game import Game
from functools import reduce

if (len(sys.argv) > 1):
    in_file = sys.argv[1]
else:
    in_file = input("Enter path to the file with the tags: ")

with open(in_file, 'r') as in_1_file:
    game_values = in_1_file.readlines()
    products_of_mins = []
    for game_line in game_values:
        g = Game(game_line)
        min_cubes_per_color = g.min_cubes_per_color()
        product_of_mins = reduce(lambda x,y: x*y, min_cubes_per_color, 1)
        print(f'for game {g.id} the minimum cubes per color are {min_cubes_per_color} - product of mins: {product_of_mins}')
        products_of_mins.append(product_of_mins)
    
    print(f'solution: {sum(products_of_mins)}')