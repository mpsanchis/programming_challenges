class Game:

    def __init__(self, game_line):
        self.id = int(game_line.split(':')[0].split(' ')[1])

        extractions_raw = game_line.split(':')[1].strip().split(';')
        self.extractions = []

        for extraction_raw in extractions_raw:
            extraction_trimmed = extraction_raw.strip()
            extraction_trimmed_split = list(map(lambda x: x.strip(), extraction_trimmed.split(',')))

            red = 0
            green = 0
            blue = 0
            for color_and_num in extraction_trimmed_split:
                [number, color] = color_and_num.split(' ')
                if color == 'red':
                    red = int(number)
                elif color == 'green':
                    green = int(number)
                elif color == 'blue':
                    blue = int(number)
                else:
                    raise Exception(f'color not allowed: {color}')
            
            self.extractions.append([red, green, blue])

    def are_extractions_valid(self, rmax, gmax, bmax):
        for e in self.extractions:
            if e[0] > rmax or e[1] > gmax or e[2] > bmax:
                return False
        
        return True

    def min_cubes_per_color(self):
        return [
            max([extraction[0] for extraction in self.extractions]),
            max([extraction[1] for extraction in self.extractions]),
            max([extraction[2] for extraction in self.extractions])
        ]