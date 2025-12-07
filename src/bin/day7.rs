use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/day7.txt");

    let lines: Vec<&str> = input.lines().collect();

    let manifold: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let starting_pos = lines
        .first()
        .unwrap()
        .chars()
        .position(|c| c == 'S')
        .unwrap();

    part1(&manifold, starting_pos);
    part2(&manifold, starting_pos);
}

fn part1(manifold: &Vec<Vec<char>>, starting_pos: usize) {
    let mut tachyons: HashSet<usize> = HashSet::from([starting_pos]);
    let mut n_splits = 0;

    for r in 1..manifold.len() - 1 {
        let mut tachyons_next_row: HashSet<usize> = HashSet::new();

        let splitters: HashSet<usize> = manifold[r + 1]
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == '^')
            .map(|(i, _)| i)
            .collect();

        for tachyon in &tachyons {
            if splitters.contains(&tachyon) {
                // split
                // TODO: Figure out if we need to handle edge cases (out of bounds)
                tachyons_next_row.insert(tachyon - 1);
                tachyons_next_row.insert(tachyon + 1);
                n_splits += 1;
            } else {
                tachyons_next_row.insert(*tachyon);
            }
        }
        tachyons = tachyons_next_row;
    }
    println!("{n_splits}");
}

fn part2(manifold: &Vec<Vec<char>>, starting_pos: usize) {
    let len = manifold[0].len();
    let mut tachyons: Vec<u64> = vec![0; len];

    tachyons[starting_pos] = 1;

    for r in 1..manifold.len() - 1 {
        let mut tachyons_next_row: Vec<u64> = vec![0; len];
        let splitters: HashSet<usize> = manifold[r + 1]
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == '^')
            .map(|(i, _)| i)
            .collect();

        for (i, n_tachyons) in tachyons.iter().enumerate() {
            if splitters.contains(&i) {
                tachyons_next_row[i + 1] += n_tachyons;
                tachyons_next_row[i - 1] += n_tachyons;
            } else {
                tachyons_next_row[i] += n_tachyons;
            }
        }
        tachyons = tachyons_next_row
    }
    let total: u64 = tachyons.iter().sum();
    println!("{total}");
}
