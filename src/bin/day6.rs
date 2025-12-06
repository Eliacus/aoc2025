use std::iter::zip;

fn main() {
    let input = include_str!("../../inputs/day6.txt");
    let lines: Vec<&str> = input.lines().collect();

    let operators: Vec<char> = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.chars().next().unwrap())
        .collect();

    let sum = part1(&lines, &operators);
    println!("Part 1: {sum}");

    let number_lines: Vec<&str> = lines[..lines.len() - 1].to_vec();
    let mut cols: Vec<String> = vec![String::new(); number_lines[0].len()];
    for line in number_lines {
        for (i, c) in line.chars().enumerate() {
            cols[i].push(c);
        }
    }
    let trimmed: Vec<&str> = cols.iter().map(|x| x.trim()).collect();

    println!("{trimmed:?}");
    let mut operator_group = 0;
    let mut operator_numbers: Vec<Vec<u64>> = Vec::new();
    operator_numbers.push(Vec::new());
    for number in trimmed {
        if number.is_empty() {
            operator_group += 1;
            operator_numbers.push(Vec::new());
        } else {
            let number: u64 = number.parse().unwrap();
            operator_numbers[operator_group].push(number);
        }
    }
    println!("{operator_numbers:?}");
    let mut result: u64 = 0;
    for (nums, op) in zip(operator_numbers, operators) {
        let mut operator_result = nums[0];
        for num in 1..nums.len() {
            operator_result = apply_operator(op, operator_result, nums[num])
        }
        result += operator_result
    }
    println!("{result}")
}

fn part1(lines: &Vec<&str>, operators: &Vec<char>) -> u64 {
    let number_lines: Vec<&str> = lines[1..lines.len() - 1].to_vec();

    let mut result_vector: Vec<u64> = lines
        .first()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    for line in number_lines {
        let nums: Vec<u64> = line
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        for (i, num) in nums.iter().enumerate() {
            result_vector[i] = apply_operator(operators[i], result_vector[i], *num);
        }
    }
    // println!("{result_vector:?}");
    result_vector.iter().sum()
}
fn apply_operator(operator: char, x: u64, y: u64) -> u64 {
    match operator {
        '+' => x + y,
        '*' => x * y,
        _ => panic!("Invalid operator"),
    }
}
