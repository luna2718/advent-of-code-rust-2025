use scan_fmt::scan_fmt;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges: Vec<(u64, u64)> = input.split(',')
        .take_while(|l| !l.is_empty())
        .map_while(|l| scan_fmt!(l, "{}-{}", u64, u64).ok())
        .collect();

    let mut sum: u64 = 0;
    for (begin, end) in ranges {
        for num in begin..=end {
            let num_str = num.to_string();
            if num_str.len() & 1 == 0 {
                let (l, r) = num_str.split_at(num_str.len() / 2);
                if l == r {
                    sum += num;
                }
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges: Vec<(u64, u64)> = input.split(',')
        .take_while(|l| !l.is_empty())
        .map_while(|l| scan_fmt!(l, "{}-{}", u64, u64).ok())
        .collect();

    let mut sum: u64 = 0;
    for (begin, end) in ranges {
        for num in begin..=end {
            let num_str = num.to_string();
            for i in 1..num_str.len() {
                if num_str.len() % i != 0 {
                    continue;
                }

                let mut valid = true;
                for n in 1..num_str.len()/i {
                    if num_str[0..i] != num_str[n*i..(n+1)*i] {
                        valid = false;
                        break;
                    } 
                }

                if valid {
                    sum += num;
                    break;
                }
            }
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
