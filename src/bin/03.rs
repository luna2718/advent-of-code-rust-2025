use std::collections::BinaryHeap;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let banks: Vec<Vec<u8>> = input.split('\n')
                    .take_while(|l| !l.is_empty())
                    .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
                    .collect();

    let mut max_sum: u64 = 0;
    for bank in banks {
        max_sum += find_best(bank, 2)?;
    }

    Some(max_sum)
}

fn find_best(bank: Vec<u8>, size: usize) -> Option<u64> {
    let mut max = *bank[..=bank.len() - size].iter().max().unwrap();

    if size == 1 {
        return Some(max as u64);
    }

    let mut real_best = 0;
    loop {
        let max_poses: Vec<usize> = bank.iter().enumerate().filter(|x| *x.1 == max && x.0 <= bank.len() - size).map(|(p, _)| p).collect();

        for pos in max_poses {
            let best = find_best(bank[pos+1..].to_vec(), size - 1);
            if let Some(best) = best && best > real_best {
                real_best = best;
            }
        }

        if real_best != 0 {
            break;
        }

        max -= 1;
    }


    Some(10u64.pow(size as u32 - 1) * max as u64 + real_best) 
}

pub fn part_two(input: &str) -> Option<u64> {
    let banks: Vec<Vec<u8>> = input.split('\n')
                    .take_while(|l| !l.is_empty())
                    .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
                    .collect();

    let mut max_sum: u64 = 0;
    for bank in banks {
        max_sum += find_best(bank, 12)?;
    }

    Some(max_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619u64));
    }
}
