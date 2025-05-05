class Scratchcard:
    
    def __init__(self, input_line):
        self.id = input_line.split(':')[0].split(' ')[1]
        numbers = input_line.split(':')[1]
        winning_numbers = [x for x in numbers.split('|')[0].strip().split(' ') if x != '']
        self.winning_numbers = [int(x) for x in winning_numbers]
        numbers_i_have = [x for x in numbers.split('|')[1].strip().split(' ') if x != '']
        self.numbers_i_have = [int(x) for x in numbers_i_have]

    def get_points(self):
        nums_in_winning = sum([1 for x in self.numbers_i_have if x in self.winning_numbers])

        if (nums_in_winning == 0):
            return 0

        return 2**(nums_in_winning - 1)

        

    
