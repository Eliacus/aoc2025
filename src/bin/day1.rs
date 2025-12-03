fn main() {
    let input = include_str!("../../inputs/day1.txt");

    let lines = input.lines();

    let mut position = 50;
    let mut count = 0;
    for line in lines {
        let (direction_str, distance_str) = line.split_at(1);

        let direction = direction_str.as_bytes()[0] as char;

        let distance_raw: i32 = distance_str.parse().unwrap();

        let num_cycles = distance_raw / 100;

        count = count + num_cycles;

        let distance = distance_raw % 100;
        match direction {
            'L' => {
                if distance > position {
                    if position != 0 {
                        count = count + 1;
                    }
                    position = 100 - (distance - position);
                } else {
                    position = position - distance;
                }
            }
            'R' => {
                if distance > (100 - position) {
                    if position != 0 {
                        count = count + 1;
                    }
                    position = distance - (100 - position);
                } else {
                    position = position + distance;
                }
            }
            _ => panic!("Invalid direction!"),
        }
        if position == 100 {
            position = 0
        }
        if position == 0 {
            count = count + 1
        }
        println!("{direction} {distance} -> {position} -------- {count}");
    }
    println!("{count}")
}
