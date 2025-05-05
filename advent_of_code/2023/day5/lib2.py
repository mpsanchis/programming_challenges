class Range:

    def __init__(self, start, end, modifiable):
        self.start = start
        self.end = end
        self.modifiable = modifiable

    def __str__(self):
        return f'([{self.start},{self.end}], {self.modifiable})'

    def __repr__(self):
        return f'([{self.start},{self.end}], {self.modifiable})'

class Almanac:
    
    def __init__(self, in_file):
        with open(in_file, 'r') as in_1_file:
            input_lines = in_1_file.readlines()
            
            '''
            0. seed-to-soil, 1. soil-to-fertilizer, 2. fertilizer-to-water,
            3. water-to-light, 4. light-to-temperature, 5.temperature-to-humidity,
            6. humidity-to-location
            '''
            self.transformed_ranges = []
            
            reading_values = [0,0,0,0,0,0,0]
            for input_line in input_lines:
                if (input_line.startswith('seeds')):
                    parsed_line = input_line.split(':')[1].strip().split(' ')
                    self.seed_ranges = [Range(start = int(x), end = int(x) + int(parsed_line[i+1]) - 1, modifiable = True) for i,x in enumerate(parsed_line) if i%2==0]
                    self.transformed_ranges = self.seed_ranges
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
                    self.transform_ranges(input_line)
                else:
                    # Empty line: resetting
                    for r in self.transformed_ranges:
                        r.modifiable = True
                    print(f'finished transforming: {self.vector2string(reading_values)}')
                    print(f'transformed ranges: {self.transformed_ranges}')
    
    def transform_range(self, range, input_line):
        [dest_range_start, source_range_start, tr_range_length] = [int(x) for x in input_line.split(' ')]
        source_range_end = source_range_start + tr_range_length
        dest_range_end   = dest_range_start + tr_range_length
        is_start_in = (range.start >= source_range_start and range.start < source_range_end)
        is_end_in   = (range.end >= source_range_start and range.end < source_range_end)

        # whole range transformed
        if (is_start_in and is_end_in):
            offset    = range.start - source_range_start
            range_len = range.end - range.start
            return [Range(
                start = dest_range_start + offset,
                end = dest_range_start + offset + range_len,
                modifiable = False
            )]
        # last bit transformed
        elif ((not is_start_in) and is_end_in):
            r1 = Range(start = range.start, end = source_range_start - 1, modifiable = True)
            offset_end = range.end - source_range_start
            r2 = Range(start = dest_range_start, end = dest_range_start + offset_end, modifiable = False)
            return [r1, r2]
        # first bit transformed
        elif (is_start_in and (not is_end_in)):
            r1 = Range(start = source_range_end, end = range.end, modifiable = True)
            offset_start = range.start - source_range_start
            r2 = Range(start = dest_range_start + offset_start, end = dest_range_end - 1, modifiable = False)
            return [r1, r2]
        # range not in transformation interval: can be all outside or part of it inside
        else:
            # all input interval is outside transformation interval
            if ((range.start >= source_range_end) or (range.end < source_range_start)):
                return [Range(
                    start = range.start,
                    end = range.end,
                    modifiable = True
                )]
            # input interval contains transformation interval inside
            else:
                r1 = Range(start = range.start, end = source_range_start - 1, modifiable = True)
                r3 = Range(start = source_range_end, end = range.end, modifiable = True)
                r2 = Range(start = dest_range_start, end = dest_range_end - 1, modifiable = False)
                return [r1, r2, r3]

    def transform_ranges(self, input_line):
        old_ranges = self.transformed_ranges
        new_ranges = []

        for r in old_ranges:
            if (r.modifiable):
                new_ranges += self.transform_range(r, input_line)
            else:
                new_ranges += [r]
        
        self.transformed_ranges = new_ranges

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


        

    
