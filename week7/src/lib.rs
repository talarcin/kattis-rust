use std::cmp::min;

pub struct Book {
    h: i32,
    w: i32,
}

pub fn bookshelfbuilding(lines: Vec<String>) -> String {
    let desc: Vec<i32> = lines[0]
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    let (n, x, y): (i32, i32, i32) = (desc[0], desc[1], desc[2]);
    let mut books: Vec<Book> = Vec::new();

    for i in 1..=n as usize {
        let book_values: Vec<i32> = lines[i]
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        books.push(Book {
            h: book_values[1],
            w: book_values[0],
        });
    }

    let sum_of_widths: i32 = books.iter().map(|book| book.w).sum();
    let max_height: i32 = books.iter().map(|book| book.h).max().unwrap();

    if max_height > y || sum_of_widths > 2 * x || (sum_of_widths > x && max_height >= y) {
        return "impossible".to_string();
    }

    let max_width = subsetsum(
        books,
        n as usize,
        x as usize,
        min(y - max_height, max_height),
    );

    if sum_of_widths - max_width >= 0 && sum_of_widths <= x {
        return "-1".to_string();
    } else if sum_of_widths - max_width <= x {
        return max_height.to_string();
    }

    return "impossible".to_string();
}

pub fn subsetsum(set: Vec<Book>, _n: usize, x: usize, height_cap: i32) -> i32 {
    let books_below_height: Vec<Book> = set
        .into_iter()
        .filter(|book| book.h <= height_cap)
        .collect();
    let mut table = vec![vec![false; books_below_height.len() + 1]; x + 1];

    println!("{}", table[0].len());

    for k in 0..=books_below_height.len() {
        for i in 0..=x {
            if k == 0 {
                table[i][k] = i == 0;
            } else {
                table[i][k] = table[i][k - 1]
                    || (i >= books_below_height[k - 1].w as usize
                        && table[i - books_below_height[k - 1].w as usize][k - 1]);
            }
        }
    }

    println!("\n\n--- Debug Info ---\n");

    for i in (0..=x).rev() {
        println!("{:?}", table[i]);
    }

    println!("\n--- Debug Info ---\n\n");

    for i in (0..=x).rev() {
        if table[i][books_below_height.len()] {
            return i as i32;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::bookshelfbuilding;

    #[test]
    fn test_sample_one() {
        let lines = vec![
            "4 4 4".to_string(),
            "3 1".to_string(),
            "1 1".to_string(),
            "1 3".to_string(),
            "2 2".to_string(),
        ];
        assert_eq!(bookshelfbuilding(lines), 3.to_string());
    }

    #[test]
    fn test_sample_two() {
        let lines = vec!["2 4 3".to_string(), "2 3".to_string(), "1 1".to_string()];
        assert_eq!(bookshelfbuilding(lines), "-1".to_string());
    }

    #[test]
    fn test_sample_three() {
        let lines = vec![
            "3 3 3".to_string(),
            "2 2".to_string(),
            "2 1".to_string(),
            "2 1".to_string(),
        ];
        assert_eq!(bookshelfbuilding(lines), "impossible".to_string());
    }

    #[test]
    fn test_all_books_same_height() {
        let lines = vec![
            "3 3 3".to_string(),
            "2 2".to_string(),
            "2 2".to_string(),
            "2 2".to_string(),
        ];
        assert_eq!(bookshelfbuilding(lines), "impossible".to_string());
    }
}
