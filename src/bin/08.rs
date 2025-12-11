use std::{sync::atomic::AtomicUsize, u64};

use scan_fmt::scan_fmt;

advent_of_code::solution!(8);

static ITERS: AtomicUsize = AtomicUsize::new(1000);

fn dist_sq(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> i64 {
    (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) + (p1.2 - p2.2).pow(2)
}

pub fn part_one(input: &str) -> Option<u64> {
    let coords: Vec<_> = input.split('\n').take_while(|l| !l.is_empty()).map(|l| scan_fmt!(l, "{},{},{}", i64, i64, i64).unwrap()).collect();

    let mut pairs: Vec<(usize, usize)> = (0..coords.len()).flat_map(|i| (i+1..coords.len()).map(|j| (i, j)).collect::<Vec<(usize, usize)>>()).collect();
    pairs.sort_by(|a, b| dist_sq(coords[a.0], coords[a.1]).cmp(&dist_sq(coords[b.0], coords[b.1])));
    // Each circuit is composed of addresses into `coords`
    let mut circuits: Vec<Vec<usize>> = coords.iter().enumerate().map(|c| vec![c.0]).collect();

    for pair in pairs[0..ITERS.load(std::sync::atomic::Ordering::Relaxed)].iter() {
        let pos0 = circuits.iter().position(|p| p.contains(&pair.0)).unwrap();
        let pos1 = circuits.iter().position(|p| p.contains(&pair.1)).unwrap();

        if pos0 != pos1 {
            let mut items = circuits[pos1].drain(0..).collect();
            circuits[pos0].append(&mut items);
        }
    }

    circuits.sort_by(|v1, v2| v1.len().cmp(&v2.len()));
    circuits.reverse();

    Some((circuits[0].len() * circuits[1].len() * circuits[2].len()) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let coords: Vec<_> = input.split('\n').take_while(|l| !l.is_empty()).map(|l| scan_fmt!(l, "{},{},{}", i64, i64, i64).unwrap()).collect();

    let mut pairs: Vec<(usize, usize)> = (0..coords.len()).flat_map(|i| (i+1..coords.len()).map(|j| (i, j)).collect::<Vec<(usize, usize)>>()).collect();
    pairs.sort_by(|a, b| dist_sq(coords[a.0], coords[a.1]).cmp(&dist_sq(coords[b.0], coords[b.1])));
    // Each circuit is composed of addresses into `coords`
    let mut circuits: Vec<Vec<usize>> = coords.iter().enumerate().map(|c| vec![c.0]).collect();

    for pair in pairs {
        let pos0 = circuits.iter().position(|p| p.contains(&pair.0)).unwrap();
        let pos1 = circuits.iter().position(|p| p.contains(&pair.1)).unwrap();

        if pos0 != pos1 {
            let mut items = circuits[pos1].drain(0..).collect();
            circuits[pos0].append(&mut items);

            if circuits[pos0].len() == coords.len() {
                return Some((coords[pair.0].0 * coords[pair.1].0) as u64);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        ITERS.store(10, std::sync::atomic::Ordering::Release);
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
