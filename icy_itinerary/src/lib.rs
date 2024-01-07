use std::collections::{HashMap, HashSet, LinkedList};

pub fn solve(lines: Vec<String>) -> String {
    let (n, _, adj) = parse_input(lines);

    // Intialisiere Listen für Schneeweg und Straßenweg und String für Startliste
    let mut start = "".to_string();
    let mut snow_list: LinkedList<usize> = LinkedList::new();
    let mut road_list: LinkedList<usize> = LinkedList::new();

    let mut path: String = String::from("");

    // Ist Knoten 1 mit 2 verbunden, existiert Straße zwischen 1 und 2, also starten wir auf der Straße
    // und fügen 1 und 2 ans Ende der Straßenliste
    if adj.get(&1).unwrap().contains(&2) {
        start = String::from("r");
        road_list.push_back(1);
        road_list.push_back(2);
    }
    // Ist Knoten 1 mit 2 nicht verbunden, existiert keine Straße zwischen 1 und 2, also starten wir auf dem Schneeweg
    // und fügen 1 und 2 ans Ende der Schneeliste
    else {
        start = String::from("s");
        snow_list.push_back(1);
        snow_list.push_back(2);
    }

    // Iteriere über alle Knoten von 3 bis n
    for z in 3..=n {
        // Wir haben die Situation road-snow
        if start == String::from("r") {
            //  x ist der letzte Knoten in der Straßenliste und y der erste Knoten in der Schneeliste
            let x = road_list.back().unwrap();
            let y = snow_list.front();

            // Wenn x mit z verbunden ist und y nicht mit z verbunden ist oder nicht existiert, füge z ans Ende der Straßenliste
            if adj.get(x).unwrap().contains(&z)
                && (y == None || !adj.get(y.unwrap()).unwrap().contains(&z))
            {
                road_list.push_back(z);
            }
            // Wenn x und y mit z verbunden sind oder y nicht existiert, füge z an das Ende der Straßenliste und y auch
            else if adj.get(x).unwrap().contains(&z)
                && (y == None || adj.get(y.unwrap()).unwrap().contains(&z))
            {
                road_list.push_back(z);
                road_list.push_back(snow_list.pop_front().unwrap());
            }
            // Wenn x nicht mit z verbunden ist und y nicht mit z verbunden oder nicht existiert ist oder nicht existiert, füge z an den Anfang der Schneeliste
            else if !adj.get(x).unwrap().contains(&z)
                && (y == None || !adj.get(y.unwrap()).unwrap().contains(&z))
            {
                snow_list.push_front(z);
            }
            // Wenn x nicht mit z verbunden ist und y mit z verbunden ist oder nicht existiert, dann
            // füge letzten Knoten von Straßenliste an Ende der Schneeliste und
            // betrachte Vorgänger von x
            else if !adj.get(x).unwrap().contains(&z)
                && (y == None || adj.get(y.unwrap()).unwrap().contains(&z))
            {
                snow_list.push_front(road_list.pop_back().unwrap());
                let pre_x = road_list.back();

                // Wenn pre_x mit z verbunden ist, füge z an das Ende der Straßenliste,
                // ansonsten füge z an den Anfang der Schneeliste
                if adj.get(pre_x.unwrap()).unwrap().contains(&z) {
                    road_list.push_back(z);
                } else {
                    snow_list.push_front(z);
                }

                // Ist nun letzter Knoten in Straßenliste 1, so füge 1 an Schneeliste vorne an und
                // setze Start auf Schneeweg
                if road_list.back().unwrap() == &1 {
                    snow_list.push_front(road_list.pop_back().unwrap());
                    start = String::from("s");
                }
            }
        }
        // Wir haben die Situation snow-road
        else {
            // x ist der erste Knoten in der Straßenliste und y der letzte Knoten in der Schneeliste
            let x = road_list.front();
            let y = snow_list.back().unwrap();

            // Wenn x mit z verbunden ist oder nicht existiert und y nicht mit z verbunden ist, füge z an den Anfang der Straßenliste
            if (x == None || adj.get(x.unwrap()).unwrap().contains(&z))
                && !adj.get(y).unwrap().contains(&z)
            {
                road_list.push_front(z);
            }
            // Wenn x mit z verbunden oder nicht existiert ist und y mit z verbunden ist, füge erst z an den Anfang der Straßenliste und dann y
            // Ist dann die Schneeliste leer, setze Start auf Straßenweg
            else if (x == None || adj.get(x.unwrap()).unwrap().contains(&z))
                && adj.get(y).unwrap().contains(&z)
            {
                road_list.push_front(z);
                road_list.push_front(snow_list.pop_back().unwrap());
                if snow_list.len() == 0 {
                    start = String::from("r");
                }
            }
            // Wenn x nicht mit z verbunden ist oder nicht existiert und y nicht mit z verbunden ist, füge z an das Ende der Schneeliste
            else if (x == None || !adj.get(x.unwrap()).unwrap().contains(&z))
                && !adj.get(y).unwrap().contains(&z)
            {
                snow_list.push_back(z);
            }
            // Wenn x nicht mit z verbunden ist oder nicht existiert und y mit z verbunden ist,
            // füge x ersten Knoten der Straßenliste an Ende der Schneeliste und
            // betrachte Nachfolger von x
            else if (x == None || !adj.get(x.unwrap()).unwrap().contains(&z))
                && adj.get(y).unwrap().contains(&z)
            {
                snow_list.push_back(road_list.pop_front().unwrap());
                let post_x = road_list.front();

                // Ist post_x mit z verbunden, füge z an den Anfang der Straßenliste,
                // ansonsten füge z ans Ende der Schneeliste
                if post_x == None || adj.get(post_x.unwrap()).unwrap().contains(&z) {
                    road_list.push_front(z);
                } else {
                    snow_list.push_back(z);
                }
            }
        }
    }

    // Erstelle abhängig von Start die Ausgabe
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
