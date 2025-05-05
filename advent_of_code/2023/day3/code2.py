import sys
from lib2 import Schematic

if (len(sys.argv) > 1):
    in_file = sys.argv[1]
else:
    in_file = input("Enter path to the file with the tags: ")

schematic = Schematic(in_file)

solution = 0
gear_ratios = []
for i in range(schematic.row_length):
    asterisk_positions_i = schematic.asterisk_positions[i]    
    for j in range(len(asterisk_positions_i)):
        nums_around_asterisk = schematic.numbers_around_asterisk(i, asterisk_positions_i[j])

        if (len(nums_around_asterisk) == 2):
            print(f'numbers around asterisk in [{i}][{asterisk_positions_i[j]}]: {nums_around_asterisk}')
            gear_ratio = nums_around_asterisk[0] * nums_around_asterisk[1]
            gear_ratios.append(gear_ratio)

print(f'gear ratios: {gear_ratios}')
print(f'solution: {sum(gear_ratios)}')