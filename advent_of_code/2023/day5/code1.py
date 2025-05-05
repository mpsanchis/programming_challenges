import sys
from lib1 import Almanac

if (len(sys.argv) > 1):
    in_file = sys.argv[1]
else:
    in_file = input("Enter path to the file with the tags: ")


a = Almanac(in_file)

print(f'finished transforming: humidity-to-location')
print(f'locations: {a.transformed_values}')
print(f'lowest location: {min(a.transformed_values)}')