
import sys

def is_num(s):
    return s in ['0','1','2','3','4','5','6','7','8','9']

def numberify(line):
    units = 0
    tens = 0
    for i in range(len(line)):
        char = line[i]
        if is_num(char):
            tens = int(char)
            break
    
    for j in range(len(line)-1,-1,-1):
        char = line[j]
        if is_num(char):
            units = int(char)
            break
    
    return 10*tens + units

if (len(sys.argv) > 1):
    in_file = sys.argv[1]
else:
    in_file = input("Enter path to the file with the tags: ")

with open(in_file, 'r') as in_1_file:
    cal_values = in_1_file.readlines()
    nums = [numberify(line) for line in cal_values]
    print(nums)
    print(sum(nums))

