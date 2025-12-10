advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let input: Vec<Vec<char>> = input.split('\n').take_while(|l| !l.is_empty()).map(|l| l.chars().collect()).collect();

    let start_pos = input[0].iter().position(|c| *c == 'S').unwrap();
    let mut beams = vec![(start_pos, 0usize)];
    
    let mut splits = 0u64;
    loop {
        let mut new_beams = vec![];

        for beam in &beams {
            if input[beam.1+1][beam.0] != '^' {
                new_beams.push((beam.0, beam.1+1));
            }
        }

        for beam in &beams {
            if input[beam.1+1][beam.0] == '^' {
                splits += 1;
                if !new_beams.contains(&(beam.0 - 1, beam.1+1)) {
                    new_beams.push((beam.0-1, beam.1+1));
                }
                if !new_beams.contains(&(beam.0 + 1, beam.1+1)) {
                    new_beams.push((beam.0+1, beam.1+1));
                }
            }
        }

        beams = new_beams;

        if beams[0].1 == input.len() - 1 {
            break;
        }
    }

    Some(splits)
}

fn count_future_splits(input: &Vec<Vec<char>>, mut pos: (usize, usize), cache: &mut Vec<((usize, usize), u64)>) -> u64 {
    if let Some((_, res)) = cache.iter().find(|a| a.0 == pos) {
        return *res;
    }

    let copy_pos = pos;
    while input[pos.1][pos.0] != '^' {
        pos.1 += 1;
        if pos.1 == input.len() {
            return 1;
        }
    }

    let res = count_future_splits(input, (pos.0-1, pos.1), cache) + count_future_splits(input, (pos.0+1, pos.1), cache);
    cache.push((copy_pos, res));

    return res;
}

pub fn part_two(input: &str) -> Option<u64> {
    let input: Vec<Vec<char>> = input.split('\n').take_while(|l| !l.is_empty()).map(|l| l.chars().collect()).collect();
    let start_pos = input[0].iter().position(|c| *c == 'S').unwrap();

    let mut cache= vec![];

    Some(count_future_splits(&input, (start_pos, 0), &mut cache) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
