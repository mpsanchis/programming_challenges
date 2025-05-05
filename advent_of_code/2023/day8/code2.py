import sys
from lib1 import DesertMap

if (len(sys.argv) > 1):
    in_file = sys.argv[1]
else:
    in_file = input("Enter path to the file with the tags: ")

def are_all_z(poss):
    for pos in poss:
        if pos[2]!='Z':
            return False

    return True

dm = DesertMap(in_file)

poss = [k for k in dm.graph.keys() if k[2]=='A']
num_steps = 0
Z_found = False

while (not Z_found):
    dir_pos = num_steps%len(dm.directions)
    dir = dm.directions[dir_pos]
    next_poss = [dm.graph[pos][dir] for pos in poss]

    print(f'at {poss}, dir={"L" if dir==0 else "R"} -> to {next_poss}')

    if are_all_z(next_poss):
        Z_found = True

    poss = next_poss
    num_steps+=1

print(f'number of steps: {num_steps}')

