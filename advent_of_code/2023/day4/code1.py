import sys
from lib1 import Scratchcard

if (len(sys.argv) > 1):
    in_file = sys.argv[1]
else:
    in_file = input("Enter path to the file with the tags: ")


with open(in_file, 'r') as in_1_file:
    scratchcard_lines = in_1_file.readlines()

    total_points = []
    for scratchcard_line in scratchcard_lines:
        s = Scratchcard(scratchcard_line)
        p = s.get_points()
        print(f'scratchcard {s.id} has {p} points')
        total_points.append(p)

    print(f'total points: {sum(total_points)}')
