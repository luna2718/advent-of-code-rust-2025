advent_of_code::solution!(4);

fn get_input(input: &str) -> Vec<Vec<bool>> {
    input.split('\n')
        .map(
            |l| l.chars()
                        .map(|c| c == '@')
                        .collect()
        ).collect()
}

fn process(rolls: &mut Vec<Vec<bool>>) -> u64 {
    let mut valid: u64 = 0;
    let mut poses = Vec::<(usize, usize)>::new();
    for (row, roll_row) in rolls.iter().enumerate() {
        for (col, roll) in roll_row.iter().enumerate() {
            if !roll {
                continue;
            }

            let mut sum = 0;
            sum += *rolls.get(row.wrapping_sub(1)).and_then(|row| row.get(col.wrapping_sub(1))).unwrap_or(&false) as u8;
            sum += *rolls.get(row.wrapping_sub(1)).and_then(|row| row.get(col)).unwrap_or(&false) as u8;
            sum += *rolls.get(row.wrapping_sub(1)).and_then(|row| row.get(col + 1)).unwrap_or(&false) as u8;
            sum += *roll_row.get(col.wrapping_sub(1)).unwrap_or(&false) as u8;
            sum += *roll_row.get(col + 1).unwrap_or(&false) as u8;
            sum += *rolls.get(row + 1).and_then(|row| row.get(col.wrapping_sub(1))).unwrap_or(&false) as u8;
            sum += *rolls.get(row + 1).and_then(|row| row.get(col)).unwrap_or(&false) as u8;
            sum += *rolls.get(row + 1).and_then(|row| row.get(col + 1)).unwrap_or(&false) as u8;

            if sum < 4 {
                valid += 1;
                poses.push((row, col));
            }
        }
    }

    for pos in poses {
        rolls[pos.0][pos.1] = false;
    }

    valid
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut rolls = get_input(input);

    let valid = process(&mut rolls);

    Some(valid)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut rolls = get_input(input);
    let mut valid = 0;
    loop {
        let this_iter = process(&mut rolls);
        if this_iter == 0 {
            break;
        }

        valid += this_iter;
    }

    Some(valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
