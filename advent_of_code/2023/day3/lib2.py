numbers = ['1','2','3']

class Schematic:

    def __init__(self, in_file):
        with open(in_file, 'r') as in_1_file:
            matrix_rows = in_1_file.readlines()

            self.matrix = []
            self.number_positions = []
            self.asterisk_positions = []
            self.row_length = len(matrix_rows[0]) - 1 # Correct the addition of \n symbol
            self.num_rows = len(matrix_rows)

            for row in matrix_rows:
                self.matrix.append(row)

                asterisk_positions_j = []
                number_positions_j = []
                
                reading_digits = row[0].isdigit()

                if (reading_digits):
                    number_positions_j.append(0)
                elif (not row[0].isdigit()) and (row[0] != '.'):
                    asterisk_positions_j.append(0)

                j = 1

                while(j < self.row_length):
                    if (row[j].isdigit() and (not reading_digits or j == self.row_length -1)):
                        number_positions_j.append(j)
                        reading_digits = True
                    
                    elif ((not row[j].isdigit()) and reading_digits):
                        number_positions_j.append(j-1)
                        reading_digits = False
                    
                    if (row[j] == '*'):
                        asterisk_positions_j.append(j)

                    j += 1
                
                self.number_positions.append(number_positions_j)
                self.asterisk_positions.append(asterisk_positions_j)

    # Given a position [i,j] with an asterisk, returns all numbers around it
    def numbers_around_asterisk(self, i, j):
        digits_around_asterisk = self.positions_digits_around_asterisk(i,j)
        return self.digits_to_numbers(digits_around_asterisk)

    # Given a position (i,j), returns all the positions around it where there are digits
    def positions_digits_around_asterisk(self, i, j):
        digits_around_me = []

        for i_x in [-1,0,1]:
            for j_x in [-1,0,1]:
                try:
                    if(self.matrix[i+i_x][j+j_x].isdigit()):
                        digits_around_me.append([i+i_x, j+j_x])
                except IndexError:
                    continue

        return digits_around_me

    # Given a position [i,j] containing a digit, it returns the position [k,l] containing the start of
    # the number that contains the digit in [i,j], and the number
    def ij2initial_ij(self, i, j):
        number_positions_i = self.number_positions[i]
        for init_pos in range(0,len(number_positions_i), 2):
            if (j <= number_positions_i[init_pos+1]):
                number_at_ij = int(''.join(self.matrix[i][number_positions_i[init_pos]:(number_positions_i[init_pos+1]+1)]))
                return ([i, init_pos], number_at_ij)

        raise Exception(f'no numbers found to which digit belongs [{i}][{j}]: ({self.matrix[i][j]})')


    # Given a list of positions [i,j] containing digits, returns the list of numbers that these positions belong to
    # If two positions have digits corresponding to the same number, this number is only returned once
    def digits_to_numbers(self, digit_position_list):
        number_positions = []
        number_list = []
        for digit_position in digit_position_list:
            (initial_position, number_at_ij) = self.ij2initial_ij(digit_position[0], digit_position[1])
            if (not (initial_position in number_positions)):
                number_positions.append(initial_position)
                number_list.append(number_at_ij)
        
        return number_list


