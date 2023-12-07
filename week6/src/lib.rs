pub fn gig_combinatorics(lines: Vec<String>) -> i64 {
    const MOD: i64 = 1000000007;
    let n: usize = lines[0].parse().unwrap();
    let songs: Vec<i32> = lines[1]
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    let mut table: Vec<Vec<i64>> = Vec::new();

    for i in 1..=n {
        let mut row: Vec<i64> = vec![0; 4];
        if songs[i - 1] == 1 {
            row[1] = (table[i - 1][1] + 1) % MOD;
        } else {
            row[1] = table[i - 1][1];
        }
        if songs[i - 1] == 2 {
            row[2] = (table[i - 1][2] * 2 + table[i - 1][1]) % MOD;
        } else {
            row[2] = table[i - 1][2];
        }
        if songs[i - 1] == 3 {
            row[3] = (table[i - 1][3] + table[i - 1][2]) % MOD;
        } else {
            row[3] = table[i - 1][3];
        }
        table.push(row);
    }

    return table[n][3];
}

#[cfg(test)]
mod tests {}
