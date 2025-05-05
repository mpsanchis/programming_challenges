import sys
from lib import Schematic

if (len(sys.argv) > 1):
    in_file = sys.argv[1]
else:
    in_file = input("Enter path to the file with the tags: ")

schematic = Schematic(in_file)

solution = 0
for r in range(schematic.num_rows):
    valid_numbers_i = schematic.valid_numbers(r)
    print(f'valid numbers row {r}: {valid_numbers_i}')
    solution += sum(valid_numbers_i)

print(f'solution: {solution}')