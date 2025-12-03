fn main() {
    let input = include_str!("../../inputs/day2.txt");
    let ranges: Vec<&str> = input.split(',').collect();
    let mut sum_part1 = 0;
    let mut sum_part2 = 0;
    for range in ranges {
        let (range_start, range_end) = range.trim().split_once('-').unwrap();
        let range_start: i64 = range_start.parse().unwrap();
        let range_end: i64 = range_end.parse().unwrap();
        for num in range_start..=range_end {
            let num_str = num.to_string();
            if part_1_match(&num_str) {
                sum_part1 = sum_part1 + num
            }
            if part_2_match(&num_str) {
                sum_part2 = sum_part2 + num
            }
        }
    }
    println!("Part 1");
    println!("{sum_part1}");

    println!("Part 2");
    println!("{sum_part2}");
}

fn part_1_match(num_str: &str) -> bool {
    let (first_half, second_half) = num_str.split_at(num_str.len() / 2);
    first_half == second_half
}

fn part_2_match(num_str: &str) -> bool {
    let len_num_str = num_str.len();
    for divider in 2..=len_num_str {
        if len_num_str % divider == 0 {
            let slice_len = len_num_str / divider;
            let substring = &num_str[..slice_len];
            let substrings: Vec<&str> = num_str.split_inclusive(substring).collect();
            let all_match = substrings.windows(2).all(|w| w[0] == w[1]);
            if all_match {
                return true;
            }
        }
    }
    return false;
}
