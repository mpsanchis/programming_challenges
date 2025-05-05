import sys
from lib2 import Almanac

if (len(sys.argv) > 1):
    in_file = sys.argv[1]
else:
    in_file = input("Enter path to the file with the tags: ")


a = Almanac(in_file)
location_ranges = a.transformed_ranges

print(f'finished transforming: humidity-to-location')
print(f'locations ranges: {location_ranges}')

lowest_location = min([r.start for r in location_ranges])

print(f'lowest location: {lowest_location}')

