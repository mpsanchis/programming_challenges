class Almanac:
    
    def __init__(self, in_file):
        with open(in_file, 'r') as in_1_file:
            input_lines = in_1_file.readlines()
            
            '''
            0. seed-to-soil, 1. soil-to-fertilizer, 2. fertilizer-to-water,
            3. water-to-light, 4. light-to-temperature, 5.temperature-to-humidity,
            6. humidity-to-location
            '''
            self.transformed_values = []
            
            reading_values = [0,0,0,0,0,0,0]
            for input_line in input_lines:
                if (input_line.startswith('seeds')):
                    self.seeds = [int(x) for x in input_line.split(':')[1].strip().split(' ')]
                    self.transformed_values = self.seeds
                    self.already_transformed = [False for x in self.transformed_values]
                elif (input_line.startswith('seed-to-soil')):
                    reading_values = [1,0,0,0,0,0,0]
                elif (input_line.startswith('soil-to-fertilizer')):
                    reading_values = [0,1,0,0,0,0,0]
                elif (input_line.startswith('fertilizer-to-water')):
                    reading_values = [0,0,1,0,0,0,0]
                elif (input_line.startswith('water-to-light')):
                    reading_values = [0,0,0,1,0,0,0]
                elif (input_line.startswith('light-to-temperature')):
                    reading_values = [0,0,0,0,1,0,0]
                elif (input_line.startswith('temperature-to-humidity')):
                    reading_values = [0,0,0,0,0,1,0]
                elif (input_line.startswith('humidity-to-location')):
                    reading_values = [0,0,0,0,0,0,1]
                elif (input_line[0].isdigit()):
                    self.transform_values(input_line, reading_values)
                else:
                    # Empty line: resetting
                    self.already_transformed = [False for x in self.transformed_values]
                    print(f'finished transforming: {self.vector2string(reading_values)}')
                    print(f'transformed values: {self.transformed_values}')
    
    def transform_values(self, input_line, reading_values):
        [dest_range_start, source_range_start, range_length] = [int(x) for x in input_line.split(' ')]
        
        old_values = self.transformed_values
        new_values = []

        for i, v in enumerate(old_values):
            if (not self.already_transformed[i]) and (v >= source_range_start) and (v < source_range_start + range_length):
                offset = v - source_range_start
                new_values.append(dest_range_start + offset)
                self.already_transformed[i] = True
            else:
                new_values.append(v)
        
        self.transformed_values = new_values

    def vector2string(self, reading_values):
        if (reading_values == [1,0,0,0,0,0,0]):
            return 'seed-to-soil'
        if (reading_values == [0,1,0,0,0,0,0]):
            return 'soil-to-fertilizer'
        if (reading_values == [0,0,1,0,0,0,0]):
            return 'fertilizer-to-water'
        if (reading_values == [0,0,0,1,0,0,0]):
            return 'water-to-light'
        if (reading_values == [0,0,0,0,1,0,0]):
            return 'light-to-temperature'
        if (reading_values == [0,0,0,0,0,1,0]):
            return 'temperature-to-humidity'
        if (reading_values == [0,0,0,0,0,0,1]):
            return 'humidity-to-location'


        

    
