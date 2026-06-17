use std::{collections::BTreeSet, fmt::Debug};

struct DayTemperature {
    day: usize,
    temperature: i32,
}

impl Ord for DayTemperature {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.temperature, self.day).cmp(&(other.temperature, other.day))
    }
}
impl PartialOrd for DayTemperature {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for DayTemperature {
    fn eq(&self, other: &Self) -> bool {
        self.temperature.eq(&other.temperature) && self.day.eq(&other.day)
    }
}
impl Eq for DayTemperature {}
impl Debug for DayTemperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("(d: {}, t: {})", self.day, self.temperature))
    }
}

static VERBOSE: bool = true;

struct Solution;
// Given an array of integers temperatures represents the daily temperatures, return an array answer such that answer[i] is the number of days you have to wait after the ith day to get a warmer temperature. If there is no future day for which this is possible, keep answer[i] == 0 instead.
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        // Edge case: no input
        if temperatures.is_empty() {
            return vec![];
        }

        let mut days_local_max_temp: BTreeSet<DayTemperature> = BTreeSet::new();
        let mut solution = vec![0_i32; temperatures.len()];

        // Initialize by marking first element as local maximum
        days_local_max_temp.insert(DayTemperature {
            day: 0,
            temperature: temperatures[0],
        });

        for (today, todays_temp) in temperatures.iter().enumerate() {
            // Check if we can mark some past temperatures as "processed", if their temp is lower
            // than today's
            let days_geq_temp = days_local_max_temp.split_off(&DayTemperature {
                day: 0,
                temperature: *todays_temp,
            });
            if VERBOSE {
                println!(
                    "Past days compared to today's ({}, {}F):",
                    today, todays_temp
                );
                println!("<  today's: {:?}", &days_local_max_temp);
                println!(">= today's: {:?}", &days_geq_temp);
            }
            // days_local_max_temp now contains days whose temperature is < temp

            // Iterate days whose temperature is < temp
            for DayTemperature {
                day: day_local_max_temp,
                temperature: _,
            } in days_local_max_temp.iter()
            {
                // Assign number of days it had to wait until today (higher temperature)
                solution[*day_local_max_temp] = (today - day_local_max_temp) as i32;
            }
            // After the days have been processed, re-assign days_local_max_temp to its original
            // meaning: "only unprocessed days"
            days_local_max_temp = days_geq_temp;
            // Add today to the list of days unprocessed
            days_local_max_temp.insert(DayTemperature {
                day: today,
                temperature: *todays_temp,
            });
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
    println!("Hello, world!");

    let mut temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
    test(temperatures, vec![1, 1, 4, 2, 1, 1, 0, 0]);

    temperatures = vec![30, 40, 50, 60];
    test(temperatures, vec![1, 1, 1, 0]);

    temperatures = vec![30, 60, 90];
    test(temperatures, vec![1, 1, 0]);

    let temperatures = vec![89, 62, 70, 58, 47, 47, 46, 76, 100, 70];
    test(temperatures, vec![8, 1, 5, 4, 3, 2, 1, 1, 0, 0]);
}
