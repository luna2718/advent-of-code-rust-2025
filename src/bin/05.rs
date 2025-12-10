use std::cmp;

use scan_fmt::scan_fmt;

fn range_int(r0: (u64, u64), r1: (u64, u64)) -> Option<(u64, u64)> {
    let range = (cmp::max(r0.0, r1.0), cmp::min(r0.1, r1.1));
    if range.0 > range.1 {
        None
    } else {
        Some(range)
    }
}

// Calculates a set theoretic r0 [r00, r01] - r1 [r10, r11]
fn range_diff(r0: (u64, u64), r1: (u64, u64)) -> Vec<(u64, u64)> {
    let int = range_int(r0, r1);
 
    match int {
        None => vec![r0],
        Some(int) => {
            if int.0 == r0.0 && int.1 == r0.1 {
                vec![]
            } else if int.0 == r0.0 {
                if int.1 < r0.1 {
                    vec![(int.1+1, r0.1)]
                } else {
                    vec![]
                }
            } else if int.1 == r0.1 {
                vec![(r0.0, int.0-1)]
            } else {
                vec![(r0.0, int.0-1), (int.1+1, r0.1)]
            }
        },
    }
}

fn diff(range: (u64, u64), ranges: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut vec = vec![range];

    for r0 in ranges {
        let mut vec_copy = vec![];
        for r1 in vec {
            let mut diff = range_diff(r1, *r0);
            vec_copy.append(&mut diff);
        }
        vec = vec_copy;
    }

    vec
}

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.split("\n").collect();

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();
    let mut in_range_section: bool = true;
    for line in lines {
        if line.is_empty() {
            in_range_section = false;
            continue;
        }

        if in_range_section {
            ranges.push(
                scan_fmt!(line, "{}-{}", u64, u64).unwrap(),
            );
        } else {
            ids.push(
                scan_fmt!(line, "{}", u64).unwrap()
            )
        }
    }

    let mut valid = 0;
    for id in ids {
        for range in &ranges {
            if range.0 <= id && id <= range.1 {
                valid += 1;
                break;
            }
        }
    }

    Some(valid)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.split("\n").collect();

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();
    let mut in_range_section: bool = true;
    for line in lines {
        if line.is_empty() {
            in_range_section = false;
            continue;
        }

        if in_range_section {
            ranges.push(
                scan_fmt!(line, "{}-{}", u64, u64).unwrap(),
            );
        } else {
            ids.push(
                scan_fmt!(line, "{}", u64).unwrap()
            )
        }
    }

    let mut fixed_ranges: Vec<(u64, u64)> = Vec::new();
    for range in ranges {
        fixed_ranges.append(&mut diff(range, &fixed_ranges))
    }

    let mut sum = 0;

    for range in fixed_ranges {
        if range.1 >= range.0 {
            sum += (range.1 - range.0) + 1;
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
