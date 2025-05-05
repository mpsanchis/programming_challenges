class DesertMap:

    def __init__(self, in_file):

        with open(in_file, 'r') as in_1_file:
            input_lines = in_1_file.readlines()

            self.graph = {}
            self.directions = []

            for input_line in input_lines:
                if '=' in input_line:
                    orig   = input_line.split('=')[0].strip()
                    dest_l = input_line.split('=')[1].strip()[1:4]
                    dest_r = input_line.split('=')[1].strip()[6:9]
                    self.graph[orig] = [dest_l, dest_r]
                elif ('L' in input_line) or ('R' in input_line):
                    for c in input_line:
                        if c == 'L':
                            self.directions.append(0)
                        elif c == 'R':
                            self.directions.append(1)
