use regex::Regex;
use std::collections::HashMap;

pub fn parse_input() -> (Vec<u32>, Vec<u32>) {
    let input = include_str!("./data.in");
    let mut l_vec = Vec::new();
    let mut r_vec = Vec::new();

    let re = Regex::new(r"(?<l>\d+)\s+(?<r>\d+)").unwrap();

    input.lines().for_each(|line| {
        let Some(captures) = re.captures(line) else {
            panic!("Could not parse input")
        };

        l_vec.push(captures["l"].parse().unwrap());
        r_vec.push(captures["r"].parse().unwrap());
    });

    l_vec.sort();
    r_vec.sort();

    (l_vec, r_vec)
}

pub fn part1() -> u32 {
    let (l_vec, r_vec) = parse_input();

    l_vec
        .iter()
        .zip(r_vec.iter())
        .fold(0, |acc, (&l, &r)| acc + l.abs_diff(r))
}

pub fn part2() -> u32 {
    let (l_vec, r_vec) = parse_input();

    let mut r_map: HashMap<u32, u32> = HashMap::new();
    r_vec.iter().for_each(|item| {
        r_map.entry(*item).and_modify(|v| *v += 1).or_insert(1);
    });

    l_vec
        .iter()
        .fold(0, |acc, item| acc + item * r_map.get(item).unwrap_or(&0))
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        assert_eq!(part1(), 1666427);
    }

    #[test]
    pub fn test_part2() {
        assert_eq!(part2(), 24316233);
    }
}
