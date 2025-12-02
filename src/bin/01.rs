advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let rots: Vec<i64> = input
        .split('\n')
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let c = l.chars().nth(0).unwrap();
            let i: i64 = l[1..].parse().unwrap();
            i * match c {
                'L' => -1,
                'R' => 1,
                _ => unreachable!()
            }
                
        })
        .into_iter()
        .collect();

    let mut sum: u64 = 0;
    let mut dial: i64 = 50;
    for rot in rots {
        dial = (dial + rot).rem_euclid(100);
        if dial == 0 {
            sum += 1;
        }
    }
    
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rots: Vec<i64> = input
        .split('\n')
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let c = l.chars().nth(0).unwrap();
            let i: i64 = l[1..].parse().unwrap();
            i * match c {
                'L' => -1,
                'R' => 1,
                _ => unreachable!()
            }
                
        })
        .into_iter()
        .collect();

    let mut sum: u64 = 0;
    let mut dial: i64 = 50;
    for rot in rots {
        for _ in 0..rot.abs() {
            dial = (dial + rot.signum()).rem_euclid(100);
            if dial == 0 {
                sum += 1;
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
