advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.split('\n').take_while(|l| !l.is_empty()).collect();
    let (last, rest) = lines.split_last()?;
    
    let mut problems: Vec<Vec<u64>> = vec![];
    for line in rest {
        for (i, el) in line.split(' ').filter(|x| !x.is_empty()).enumerate() {
            let num = el.parse::<u64>().unwrap();
            if let Some(pos) = problems.get_mut(i) {
                pos.push(num);
            } else {
                problems.push(vec![num]);
            }
        }
    }

    let mut result = 0;
    for (i, op) in last.split(' ').filter(|l| !l.is_empty()).enumerate() {
        let calculation = if op == "*" {
            let mut prod = 1;
            for j in &problems[i] {
                prod *= j;
            }
            prod
        } else {
            let mut sum = 0;
            for j in &problems[i] {
                sum += j;
            }
            sum
        };

        result += calculation;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.split('\n').take_while(|l| !l.is_empty()).collect();
    let (last, rest) = lines.split_last()?;
    
    let split_lines: Vec<Vec<u64>> = rest.iter()
                                         .map(
                                            |f| f.chars()
                                                        .map(
                                                            |f| f.to_digit(10).unwrap_or(0) as u64
                                                        ).collect()
                                            ).collect();

    let longest_line_length = lines.iter().max_by(|l1, l2| l1.len().cmp(&l2.len())).unwrap().len();
    let mut problems = vec![];
    let mut problem = vec![];
    for i in 0..longest_line_length {
        let mut value: u64 = 0;
        for line in &split_lines {
            let val = *line.get(i).unwrap_or(&0);
            if val != 0 {
                value *= 10;
                value += val;
            }
        }

        if value == 0 {
            problems.push(problem.clone());
            problem.clear();
        } else {
            problem.push(value);
        }
    }
    problems.push(problem);

    let mut result = 0;
    for (i, op) in last.split(' ').filter(|l| !l.is_empty()).enumerate() {
        let calculation = if op == "*" {
            let mut prod = 1;
            for j in &problems[i] {
                prod *= j;
            }
            prod
        } else {
            let mut sum = 0;
            for j in &problems[i] {
                sum += j;
            }
            sum
        };

        result += calculation;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
