pub fn greedy_scheduling(lines: Vec<String>) -> i32 {
    let num_intervals: usize = lines[0].parse().unwrap();
    let mut intervals: Vec<Vec<i64>> = vec![vec![0; 2]; num_intervals];

    for i in 1..=num_intervals {
        let interval: Vec<i64> = lines[i]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        intervals[i - 1] = interval;
    }

    intervals.sort_by(|a, b| a[1].cmp(&b[1]));
    let mut counter: i32 = 1;
    let mut latest_end: i64 = intervals[0][1];

    for i in 1..intervals.len() {
        if intervals[i][0] >= latest_end {
            counter += 1;
            latest_end = intervals[i][1];
        }
    }

    counter
}

pub fn greedy_frosh_week(lines: Vec<String>) -> i32 {
    let desc: Vec<usize> = lines[0]
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    let _n = desc[0];
    let m = desc[1];

    let mut task_lengths: Vec<i32> = lines[1]
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    let mut silence_lengths: Vec<i32> = lines[2]
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    // Sort and reverse to achieve sorting by descending order
    task_lengths.sort();
    task_lengths.reverse();
    silence_lengths.sort();
    silence_lengths.reverse();

    let mut idx: usize = 0;
    let mut counter: i32 = 0;

    for task_length in task_lengths {
        if task_length <= silence_lengths[idx] {
            counter += 1;
            idx = if idx + 1 == m {
                break;
            } else {
                idx + 1
            };
        }
    }

    return counter;
}

pub fn greedy_planting(lines: Vec<String>) -> i32 {
    let _n: usize = lines[0].parse().unwrap();
    let mut times: Vec<i32> = lines[1]
        .split_whitespace()
        .map(|t| t.parse().unwrap())
        .collect();

    times.sort();
    times.reverse();

    let mut wait: i32 = 0;
    let mut day: i32 = 1;

    for time in times {
        if time > wait - day {
            wait = wait + time - (wait - day);
        }
        day += 1;
    }

    return wait + 1;
}

pub fn greedy_discount(lines: Vec<String>) -> i64 {
    let n: i32 = lines[0].parse().unwrap();

    if n < 3 {
        return 0;
    }

    let mut p: Vec<i64> = lines[1]
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    p.sort();
    p.reverse();
    let mut max_discount: i64 = 0;

    for i in 0..p.len() {
        if (i + 1) % 3 == 0 {
            max_discount += p[i];
        }
    }

    max_discount
}

#[cfg(test)]
mod tests {
    use crate::{greedy_discount, greedy_frosh_week, greedy_scheduling};

    #[test]
    fn simple_test_greedy_scheduling() {
        let lines = vec![
            "3".to_string(),
            "1 4".to_string(),
            "2 8".to_string(),
            "5 9".to_string(),
        ];
        assert_eq!(greedy_scheduling(lines), 2);
    }

    #[test]
    fn longer_test_greedy_scheduling() {
        let lines = vec![
            "8".to_string(),
            "0 6".to_string(),
            "1 4".to_string(),
            "3 5".to_string(),
            "3 8".to_string(),
            "4 7".to_string(),
            "5 9".to_string(),
            "6 10".to_string(),
            "8 11".to_string(),
        ];

        assert_eq!(greedy_scheduling(lines), 3);
    }

    #[test]
    fn test_greedy_frosh() {
        let lines = vec![
            "5 4".to_string(),
            "150000 100000 160000 100000 180000".to_string(),
            "190000 170000 140000 160000".to_string(),
        ];
        assert_eq!(greedy_frosh_week(lines), 4);

        let lines_two = vec![
            "4 4".to_string(),
            "180000 185000 199999 100000".to_string(),
            "199999 180000 170000 120000".to_string(),
        ];
        assert_eq!(greedy_frosh_week(lines_two), 3);

        let lines_three = vec![
            "3 3".to_string(),
            "199999 180000 170001".to_string(),
            "199999 170000 180000".to_string(),
        ];

        assert_eq!(greedy_frosh_week(lines_three), 2);
    }

    #[test]
    fn test_greedy_discount() {
        let input = vec!["7".to_string(), "400 350 200 250 300 150 100".to_string()];
        assert_eq!(greedy_discount(input), 450);
        let input_two = vec!["2".to_string(), "400 350".to_string()];
        assert_eq!(greedy_discount(input_two), 0);
        let input_three = vec!["3".to_string(), "400 350 100".to_string()];
        assert_eq!(greedy_discount(input_three), 100);
    }
}
