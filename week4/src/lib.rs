use std::vec;

pub fn firefly(lines: Vec<String>) -> () {
    let desc: Vec<i32> = lines[0]
        .split_whitespace()
        .map(|l| l.parse().unwrap())
        .collect();
    let n = desc[0];
    let h = desc[1];

    let mut upper: Vec<i32> = Vec::new();
    let mut lower: Vec<i32> = Vec::new();
    let mut crashes: Vec<i32> = vec![0; h as usize];

    for i in 1..=n {
        if (i - 1) % 2 == 0 {
            lower.push(lines[i as usize].parse().unwrap());
        } else {
            upper.push(lines[i as usize].parse().unwrap());
        }
    }

    lower.sort();
    upper.sort();
    upper.reverse();
    let mut idx = 0;

    for height in 1..=h {
        for i in idx..lower.len() {
            if height <= lower[i] {
                idx = i;
                crashes[(height - 1) as usize] += (lower.len() - idx) as i32;
                break;
            }
        }
    }

    idx = 0;

    for height in 1..=h {
        for i in idx..upper.len() {
            if height <= h - upper[i] {
                idx = i;
                crashes[(height - 1) as usize] += idx as i32;
                break;
            } else if i == upper.len() - 1 {
                crashes[(height - 1) as usize] += upper.len() as i32;
            }
        }
    }

    let min = crashes.iter().min().unwrap();
    let mut min_levels = 0;

    for i in 0..crashes.len() {
        if crashes[i] == *min {
            min_levels += 1;
        }
    }

    println!("{} {}", *min, min_levels)
}

pub fn free_weights(lines: Vec<String>) -> i32 {
    let n: usize = lines[0].parse().unwrap();
    const MAX_WEIGHT: i32 = 1000000000;

    let first_row: Vec<i32> = lines[1]
        .split_whitespace()
        .map(|w| w.parse().unwrap())
        .collect();
    let second_row: Vec<i32> = lines[2]
        .split_whitespace()
        .map(|w| w.parse().unwrap())
        .collect();

    let rows = vec![first_row, second_row];

    let mut low: i32 = 0;
    let mut high: i32 = MAX_WEIGHT;

    while low <= high {
        let middle: i32 = (low + high) / 2;
        if sortable_with_max_weight(middle, &rows, n) {
            high = middle - 1;
        } else {
            low = middle + 1;
        }
    }

    low
}

fn sortable_with_max_weight(max_weight: i32, rows: &Vec<Vec<i32>>, num_pairs: usize) -> bool {
    let mut unpaired: i32 = -1;

    for i in 0..num_pairs {
        if rows[0][i] > max_weight {
            if unpaired == -1 {
                unpaired = rows[0][i];
            } else if unpaired == rows[0][i] {
                unpaired = -1;
            } else {
                return false;
            }
        }
    }

    if unpaired != -1 {
        return false;
    }

    for i in 0..num_pairs {
        if rows[1][i] > max_weight {
            if unpaired == -1 {
                unpaired = rows[1][i];
            } else if unpaired == rows[1][i] {
                unpaired = -1;
            } else {
                return false;
            }
        }
    }

    return true;
}

pub fn room_painting(lines: Vec<String>) -> i64 {
    // Parse input description
    let desc: Vec<i32> = lines[0]
        .split_whitespace()
        .map(|l| l.parse().unwrap())
        .collect();
    let n = desc[0];
    let m = desc[1];

    let mut can_sizes: Vec<i32> = Vec::new();
    let mut needed_amounts: Vec<i32> = Vec::new();

    // Parse input in vectors for better handling
    for i in 1..=n {
        can_sizes.push(lines[i as usize].parse().unwrap());
    }
    for i in n + 1..=n + m {
        needed_amounts.push(lines[i as usize].parse().unwrap());
    }

    let mut wasted: i64 = 0;
    needed_amounts.sort();
    can_sizes.sort();
    let mut idx: usize = 0;

    for i in 0..needed_amounts.len() {
        for j in idx..=can_sizes.len() {
            if needed_amounts[i] <= can_sizes[j] {
                idx = j;
                wasted += (can_sizes[j] - needed_amounts[i]) as i64;
                break;
            }
        }
        if needed_amounts[i] == 0 {
            wasted -= can_sizes[idx] as i64;
        }
    }

    return wasted;
}

#[cfg(test)]
mod tests {
    use crate::room_painting;

    #[test]
    fn test_room_painting() {
        let lines = vec![
            "1 5".to_string(),
            "1000000000".to_string(),
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
        ];
        assert_eq!(room_painting(lines), 5 * (1000000000 - 1));
    }
}
