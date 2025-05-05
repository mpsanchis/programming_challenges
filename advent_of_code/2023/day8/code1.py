import sys
from lib1 import DesertMap

if (len(sys.argv) > 1):
    in_file = sys.argv[1]
else:
    in_file = input("Enter path to the file with the tags: ")


dm = DesertMap(in_file)

pos = 'AAA'
num_steps = 0
ZZZ_found = False

while (not ZZZ_found):
    dir_pos = num_steps%len(dm.directions)
    dir = dm.directions[dir_pos]
    next_pos = dm.graph[pos][dir]

    print(f'at {pos}, dir={"L" if dir==0 else "R"} -> to {next_pos}')

    if next_pos == 'ZZZ':
        ZZZ_found = True

    pos = next_pos
    num_steps+=1

print(f'number of steps: {num_steps}')

