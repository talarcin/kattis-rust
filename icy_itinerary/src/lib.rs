use std::collections::{HashMap, HashSet, LinkedList};

pub fn solve(lines: Vec<String>) -> String {
    let (n, _, adj) = parse_input(lines);

    let mut snow_list: LinkedList<usize> = LinkedList::new();
    let mut road_list: LinkedList<usize> = LinkedList::new();

    let mut path: String = String::from("");

    // Wir haben die Situation road-snow
    if adj.get(&1).unwrap().contains(&2) {
        road_list.push_back(1);
        road_list.push_back(2);

        for z in 3..=n {
            let x = road_list.back().unwrap();
            let y = snow_list.front();

            if adj.get(x).unwrap().contains(&z)
                && (y == None || !adj.get(y.unwrap()).unwrap().contains(&z))
            {
                road_list.push_back(z);
            } else if adj.get(x).unwrap().contains(&z)
                && (y == None || adj.get(y.unwrap()).unwrap().contains(&z))
            {
                road_list.push_back(z);
                road_list.push_back(snow_list.pop_front().unwrap());
            } else if !adj.get(x).unwrap().contains(&z)
                && (y == None || !adj.get(y.unwrap()).unwrap().contains(&z))
            {
                snow_list.push_front(z);
            } else if !adj.get(x).unwrap().contains(&z)
                && (y == None || adj.get(y.unwrap()).unwrap().contains(&z))
            {
                snow_list.push_front(road_list.pop_back().unwrap());
                let pre_x = road_list.back();

                if adj.get(pre_x.unwrap()).unwrap().contains(&z) {
                    road_list.push_back(z);
                } else {
                    snow_list.push_front(z);
                }
            }
        }

        path = make_path(&road_list, &snow_list);
    }
    // Wir haben die Situation snow-road
    else {
        snow_list.push_back(1);
        snow_list.push_back(2);

        for z in 3..=n {
            let x = road_list.front();
            let y = snow_list.back().unwrap();

            if (x == None || adj.get(x.unwrap()).unwrap().contains(&z))
                && !adj.get(y).unwrap().contains(&z)
            {
                road_list.push_front(z);
            } else if (x == None || adj.get(x.unwrap()).unwrap().contains(&z))
                && adj.get(y).unwrap().contains(&z)
            {
                road_list.push_front(z);
                road_list.push_front(snow_list.pop_back().unwrap());
            } else if (x == None || !adj.get(x.unwrap()).unwrap().contains(&z))
                && !adj.get(y).unwrap().contains(&z)
            {
                snow_list.push_back(z);
            } else if (x == None || !adj.get(x.unwrap()).unwrap().contains(&z))
                && adj.get(y).unwrap().contains(&z)
            {
                snow_list.push_back(road_list.pop_front().unwrap());
                let post_x = road_list.front().unwrap();

                if adj.get(post_x).unwrap().contains(&z) {
                    road_list.push_front(z);
                } else {
                    snow_list.push_back(z);
                }
            }
        }

        path = make_path(&snow_list, &road_list);
    }

    path
}

pub fn parse_input(lines: Vec<String>) -> (usize, usize, HashMap<usize, HashSet<usize>>) {
    let desc: Vec<usize> = lines[0]
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let mut adj: HashMap<usize, HashSet<usize>> = HashMap::new();

    for i in 1..=desc[0] {
        adj.insert(i, HashSet::new());
    }

    for i in 1..=desc[1] {
        let pair: Vec<usize> = lines[i]
            .split_whitespace()
            .map(|l| l.parse().unwrap())
            .collect();

        adj.get_mut(&pair[0]).unwrap().insert(pair[1]);
        adj.get_mut(&pair[1]).unwrap().insert(pair[0]);
    }

    return (desc[0], desc[1], adj);
}

pub fn make_path(front: &LinkedList<usize>, back: &LinkedList<usize>) -> String {
    let mut path = String::from("");

    for h in front {
        path.push_str(&h.to_string());
        path.push_str(" ");
    }

    for h in back {
        path.push_str(&h.to_string());
        path.push_str(" ");
    }

    path.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;

    use crate::{make_path, solve};

    #[test]
    fn test_make_path() {
        let front: LinkedList<usize> = LinkedList::from([1, 2, 3]);
        let back: LinkedList<usize> = LinkedList::from([4, 5, 6]);

        assert_eq!(make_path(&front, &back), String::from("1 2 3 4 5 6"));
    }

    #[test]
    fn test_solve() {
        let inputs = vec![
            (
                vec![
                    "4 4".to_string(),
                    "1 2".to_string(),
                    "1 3".to_string(),
                    "1 4".to_string(),
                    "3 4".to_string(),
                ],
                "1 4 2 3".to_string(),
            ),
            (vec!["5 0".to_string()], "1 2 4 5 3".to_string()),
            (
                vec![
                    "5 3".to_string(),
                    "2 3".to_string(),
                    "3 4".to_string(),
                    "3 5".to_string(),
                ],
                "1 4 5 2 3".to_string(),
            ),
            (
                vec![
                    "5 4".to_string(),
                    "1 2".to_string(),
                    "2 3".to_string(),
                    "3 4".to_string(),
                    "4 5".to_string(),
                ],
                "1 2 3 4 5".to_string(),
            ),
        ];

        for (lines, expected) in inputs {
            assert_eq!(solve(lines), expected);
        }
    }
}
