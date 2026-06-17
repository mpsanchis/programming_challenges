struct Solution;
type Day = usize;
// Given an array of integers temperatures represents the daily temperatures, return an array answer such that answer[i] is the number of days you have to wait after the ith day to get a warmer temperature. If there is no future day for which this is possible, keep answer[i] == 0 instead.
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        // Edge case: no input
        if temperatures.is_empty() {
            return vec![];
        }

        let mut days_local_max_temp: Vec<Day> = Vec::new();
        let mut solution = vec![0_i32; temperatures.len()];

        for today in 0..temperatures.len() {
            let todays_temp = temperatures[today];
            // Check if we can mark some past temperatures as "processed", if their temp is lower
            // than today's
            while let Some(day_max_temp) = days_local_max_temp.last() {
                if temperatures[*day_max_temp] >= todays_temp {
                    break;
                }
                let day = *day_max_temp;
                days_local_max_temp.pop();
                solution[day] = (today - day) as i32;
            }
            days_local_max_temp.push(today);
        }
        solution
    }
}

fn test(temperatures: Vec<i32>, expected: Vec<i32>) {
    let actual = Solution::daily_temperatures(temperatures);
    for i in 0..actual.len() {
        assert_eq!(
            expected[i], actual[i],
            "\nDay {i} did not match\nexpected: {:?}\nactual:   {:?}",
            expected, actual
        );
    }
}

fn main() {
    let mut temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
    test(temperatures, vec![1, 1, 4, 2, 1, 1, 0, 0]);

    temperatures = vec![30, 40, 50, 60];
    test(temperatures, vec![1, 1, 1, 0]);

    temperatures = vec![30, 60, 90];
    test(temperatures, vec![1, 1, 0]);

    let temperatures = vec![89, 62, 70, 58, 47, 47, 46, 76, 100, 70];
    test(temperatures, vec![8, 1, 5, 4, 3, 2, 1, 1, 0, 0]);
}
