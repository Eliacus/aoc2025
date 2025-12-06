use std::cmp::max;

fn main() {
    let input = include_str!("../../inputs/day5.txt");
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();

    let fresh_ranges = build_fresh_ranges(ranges);

    let n_fresh = part1(&fresh_ranges, ingredients);

    println!("{n_fresh}");

    let total_fresh = part2(fresh_ranges);
    println!("{total_fresh}");
}

fn build_fresh_ranges(ranges: &str) -> Vec<(usize, usize)> {
    let mut fresh_ranges = Vec::new();
    for line in ranges.lines() {
        let (range_start, range_end) = line.split_once("-").unwrap();
        let range_start: usize = range_start.parse().unwrap();
        let range_end: usize = range_end.parse().unwrap();

        fresh_ranges.push((range_start, range_end));
    }

    let l = fresh_ranges.len();
    println!("num ranges: {l:?}");
    fresh_ranges
}

fn part1(fresh_ranges: &Vec<(usize, usize)>, ingredients: &str) -> i32 {
    ingredients.lines().fold(0, |acc, id| {
        let id: usize = id.parse().unwrap();
        let mut found = 0;
        for (range_start, range_end) in fresh_ranges.iter() {
            if (id >= *range_start) && (id <= *range_end) {
                found = 1;
            }
        }
        return acc + found;
    })
}

/// Sort and merge the ranges. Then compute number of elements in each range and acc
fn part2(mut fresh_ranges: Vec<(usize, usize)>) -> usize {
    let mut total = 0;
    fresh_ranges.sort();
    let mut merged = vec![];
    let mut current_range = fresh_ranges[0];
    for i in 1..fresh_ranges.len() {
        let next_range = fresh_ranges[i];
        // println!("{current_range:?} vs {next_range:?}");
        if current_range.1 >= next_range.0 {
            current_range.1 = max(current_range.1, next_range.1);
        } else {
            // println!("pushing {current_range:?}");
            merged.push(current_range);
            current_range = next_range;
        }
    }
    merged.push(current_range);
    // println!("-------");
    // println!("{merged:?}");

    for range in merged {
        total += range.1 - range.0 + 1
    }

    total
}
