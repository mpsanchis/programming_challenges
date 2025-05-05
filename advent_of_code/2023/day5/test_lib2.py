from lib2 import Range, Almanac

a = Almanac('in_short.txt')

r = Range(9, 11, True)
l = '50 7 4'

sol = a.transform_range(r, l)

print('sol:')
for s in sol:
    print(s)

'''
try
water-to-light map:
88 18 7
18 25 70

with input:
[([81,94], True), ([61,69], True), ([53,56], True)]
'''