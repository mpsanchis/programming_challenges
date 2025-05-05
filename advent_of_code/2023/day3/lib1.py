numbers = ['1','2','3']

class Schematic:

    def __init__(self, in_file):
        with open(in_file, 'r') as in_1_file:
            matrix_rows = in_1_file.readlines()

            self.matrix = []
            self.number_positions = []
            self.symbol_positions = []
            self.row_length = len(matrix_rows[0]) - 1 # Correct the addition of \n symbol
            self.num_rows = len(matrix_rows)

            for row in matrix_rows:
                self.matrix.append(row)

                symbol_positions_j = []
                number_positions_j = []
                
                reading_digits = row[0].isdigit()

                if (reading_digits):
                    number_positions_j.append(0)
                elif (not row[0].isdigit()) and (row[0] != '.'):
                    symbol_positions_j.append(0)

                j = 1

                while(j < self.row_length):
                    if (row[j].isdigit() and (not reading_digits or j == self.row_length -1)):
                        number_positions_j.append(j)
                        reading_digits = True
                    
                    elif ((not row[j].isdigit()) and reading_digits):
                        number_positions_j.append(j-1)
                        reading_digits = False
                    
                    if ((not row[j].isdigit()) and (row[j] != '.')):
                        symbol_positions_j.append(j)

                    j += 1
                
                self.number_positions.append(number_positions_j)
                self.symbol_positions.append(symbol_positions_j)

    def valid_numbers(self, row_num):
        valid_numbers_i = []
        row_numbers = self.number_positions[row_num]

        for j in range(0, len(row_numbers)-1, 2):
            if (self.is_symbol_adjacent(row_num, row_numbers[j], row_numbers[j+1])):
                current_num = int(''.join(self.matrix[row_num][row_numbers[j]:(row_numbers[j+1]+1)]))
                valid_numbers_i.append(current_num)

        return valid_numbers_i
                
    def is_symbol_adjacent(self, row_num, num_start, num_end):
        # Check previous row
        if (row_num != 0):
            if ( any(symbol in range(num_start-1, num_end+2) for symbol in self.symbol_positions[row_num-1]) ):
                return True

        # Check current row
        if (((num_start - 1) in self.symbol_positions[row_num]) or ((num_end + 1) in self.symbol_positions[row_num])):
            return True

        # Check next row
        if (row_num != self.num_rows - 1):
            if (any(symbol in range(num_start-1, num_end+2) for symbol in self.symbol_positions[row_num+1])):
                return True

        return False


    





