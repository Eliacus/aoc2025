fn main() {
    let input = include_str!("../../inputs/day3.txt");
    let banks = input.lines();
    let mut sum_part1 = 0;
    let mut sum_part2 = 0;
    for bank in banks {
        let numbers: Vec<char> = bank.chars().collect();

        sum_part1 += part1(&numbers);
        sum_part2 += part2(&numbers);
    }
    println!("{sum_part1}");
    println!("{sum_part2}");
}

fn part1(numbers: &[char]) -> i32 {
    let max_idx = find_max_num(&numbers, 0, numbers.len() - 1);

    let max_idx_second = find_max_num(&numbers, max_idx + 1, numbers.len());

    let max_num_str = format!("{}{}", numbers[max_idx], numbers[max_idx_second]);
    max_num_str.parse().unwrap()
}

fn part2(numbers: &[char]) -> u64 {
    let mut target_idxs: Vec<usize> = Vec::with_capacity(12);
    for target_idx in 0..12 {
        let start;
        if target_idx == 0 {
            start = 0;
        } else {
            start = target_idxs[target_idx - 1] + 1;
        }
        let end = numbers.len() - 12 + target_idx + 1;
        // println!("start: {start}, end: {end}");
        let idx = find_max_num(numbers, start, end);
        target_idxs.push(idx);
        // println!("{target_idxs:?}");
    }
    let joltage_str: String = target_idxs
        .iter()
        .copied()
        .map(|idx| numbers[idx])
        .collect();

    joltage_str.trim().parse::<u64>().unwrap()
}

/// Finds the maximum number and it's corresponding index between start and end
fn find_max_num(numbers: &[char], start: usize, end: usize) -> usize {
    let mut max_number = 0;
    let mut max_idx: usize = 0;
    for idx in start..end {
        let num = numbers[idx].to_digit(10).unwrap();
        if num > max_number {
            max_number = num;
            max_idx = idx;
        }
    }
    max_idx
}
