use std::collections::{HashMap, HashSet, LinkedList};

pub fn solve(lines: Vec<String>) -> String {
    let (n, _, adj) = parse_input(lines);
    let mut start = "".to_string();
    let mut snow_list: LinkedList<usize> = LinkedList::new();
    let mut road_list: LinkedList<usize> = LinkedList::new();
    let mut path: String = String::from("");

    // Is node 1 connected with 2, i.e. there is a street between them,
    // then we start on a road path and push 1 and 2 to the end of road_list
    // Other wise we start on a snow path and push
    // 1 and 2 to the end of snow_list
    if adj.get(&1).unwrap().contains(&2) {
        start = String::from("r");
        road_list.push_back(1);
        road_list.push_back(2);
    } else {
        start = String::from("s");
        snow_list.push_back(1);
        snow_list.push_back(2);
    }

    // Iterate over all nodes 3 to n
    for z in 3..=n {
        // Here we have the situation road-snow
        if start == String::from("r") {
            // x is the last node of the road list and
            // y the first node of the snow list
            let x = road_list.back().unwrap();
            let y = snow_list.front();

            // If x is connected with z and
            // y not we push z to the end of road_list
            if adj.get(x).unwrap().contains(&z)
                && (y == None || !adj.get(y.unwrap()).unwrap().contains(&z))
            {
                road_list.push_back(z);
            }
            // If x and y are connected with z
            // we put z at the end of road_list and y also
            else if adj.get(x).unwrap().contains(&z)
                && (y == None || adj.get(y.unwrap()).unwrap().contains(&z))
            {
                road_list.push_back(z);
                road_list.push_back(snow_list.pop_front().unwrap());
            }
            // If x is neither connected with z nor with y
            // we push z to the beginning of snow_list
            else if !adj.get(x).unwrap().contains(&z)
                && (y == None || !adj.get(y.unwrap()).unwrap().contains(&z))
            {
                snow_list.push_front(z);
            }
            // If x is not connected with z but y is
            // we pop x from road_list and
            // push it to the beginning of snow_list
            // and look at the precursor node of x
            else if !adj.get(x).unwrap().contains(&z)
                && (y == None || adj.get(y.unwrap()).unwrap().contains(&z))
            {
                snow_list.push_front(road_list.pop_back().unwrap());
                let pre_x = road_list.back();

                // If pre_x is connected with z we push z to
                // the end of road_list, otherwise we push it
                // to the front of snow_list
                if adj.get(pre_x.unwrap()).unwrap().contains(&z) {
                    road_list.push_back(z);
                } else {
                    snow_list.push_front(z);
                }

                // In case the remaining node of road_list is 1
                // we switch the situation to snow-road
                if road_list.back().unwrap() == &1 {
                    snow_list.push_front(road_list.pop_back().unwrap());
                    start = String::from("s");
                }
            }
        }
        // The situation is snow-road
        else {
            // x ist the first node of snow_list and y the
            // last node of road_list
            let x = road_list.front();
            let y = snow_list.back().unwrap();

            // If x is connected with z and y not, we push z to the start of road_list
            if (x == None || adj.get(x.unwrap()).unwrap().contains(&z))
                && !adj.get(y).unwrap().contains(&z)
            {
                road_list.push_front(z);
            }
            // If x is connect with z and y also,
            // we push z to start of road_list and then y
            else if (x == None || adj.get(x.unwrap()).unwrap().contains(&z))
                && adj.get(y).unwrap().contains(&z)
            {
                road_list.push_front(z);
                road_list.push_front(snow_list.pop_back().unwrap());

                // If the snow list is empty, we switch to situation
                if snow_list.len() == 0 {
                    start = String::from("r");
                }
            }
            // If x and y are not connected with z, we push z to the end of snow_list
            else if (x == None || !adj.get(x.unwrap()).unwrap().contains(&z))
                && !adj.get(y).unwrap().contains(&z)
            {
                snow_list.push_back(z);
            }
            // If x ist not connected with z but y is,
            // we push x to end of snow_list and then look at node after x
            else if (x == None || !adj.get(x.unwrap()).unwrap().contains(&z))
                && adj.get(y).unwrap().contains(&z)
            {
                snow_list.push_back(road_list.pop_front().unwrap());
                let post_x = road_list.front();

                // If post_x is connected with z, we push z to start of road_list
                // otherwise to the end of snow_list
                if post_x == None || adj.get(post_x.unwrap()).unwrap().contains(&z) {
                    road_list.push_front(z);
                } else {
                    snow_list.push_back(z);
                }
            }
        }
    }

    // Generate output path based on start situation
    if start == String::from("r") {
        path = make_path(&road_list, &snow_list);
    } else {
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
            (
                vec!["2 1".to_string(), "1 2".to_string()],
                "1 2".to_string(),
            ),
            (
                vec![
                    "5 3".to_string(),
                    "1 2".to_string(),
                    "3 4".to_string(),
                    "4 5".to_string(),
                ],
                "1 4 2 3 5".to_string(),
            ),
            (
                vec!["5 1".to_string(), "2 3".to_string()],
                "1 4 5 2 3".to_string(),
            ),
            (
                vec![
                    "6 6".to_string(),
                    "1 2".to_string(),
                    "1 3".to_string(),
                    "2 3".to_string(),
                    "2 4".to_string(),
                    "2 6".to_string(),
                    "4 5".to_string(),
                ],
                "1 2 6 5 3 4".to_string(),
            ),
            (
                vec![
                    "6 4".to_string(),
                    "1 3".to_string(),
                    "2 4".to_string(),
                    "2 6".to_string(),
                    "4 5".to_string(),
                ],
                "1 2 3 6 5 4".to_string(),
            ),
            (
                vec![
                    "5 10".to_string(),
                    "1 2".to_string(),
                    "1 3".to_string(),
                    "1 4".to_string(),
                    "1 5".to_string(),
                    "2 3".to_string(),
                    "2 4".to_string(),
                    "2 5".to_string(),
                    "3 4".to_string(),
                    "3 5".to_string(),
                    "4 5".to_string(),
                ],
                "1 2 3 4 5".to_string(),
            ),
            (
                vec![
                    "9 9".to_string(),
                    "1 2".to_string(),
                    "1 9".to_string(),
                    "2 9".to_string(),
                    "3 4".to_string(),
                    "3 5".to_string(),
                    "4 5".to_string(),
                    "6 7".to_string(),
                    "6 8".to_string(),
                    "7 8".to_string(),
                ],
                "1 4 2 6 3 7 5 9 8".to_string(),
            ),
            (
                vec![
                    "7 6".to_string(),
                    "1 4".to_string(),
                    "2 3".to_string(),
                    "2 4".to_string(),
                    "5 6".to_string(),
                    "6 7".to_string(),
                    "5 7".to_string(),
                ],
                "1 4 7 2 6 3 5".to_string(),
            ),
        ];

        for (lines, expected) in inputs {
            assert_eq!(solve(lines), expected);
        }
    }
}
