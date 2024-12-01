use regex::Regex;
use std::collections::HashMap;

pub fn read_input() -> String {
    String::from(include_str!("./data.in"))
}

pub fn parse_input(input: String) -> (Vec<i32>, Vec<i32>) {
    let mut l_vec = Vec::new();
    let mut r_vec = Vec::new();
    
    let re = Regex::new(r"(?<l>\d+)\s+(?<r>\d+)").unwrap();

    for line in input.lines() {
        let Some(captures) = re.captures(line) else {
            panic!("Could not parse input")
        };
        
        l_vec.push(captures["l"].parse::<i32>().unwrap());
        r_vec.push(captures["r"].parse::<i32>().unwrap());
    }
    
    l_vec.sort();
    r_vec.sort();
    
    (l_vec, r_vec)
}

pub fn part1() -> i32 {
    let input = read_input();
    let (l_vec, r_vec) = parse_input(input);
    
    l_vec.iter().zip(r_vec.iter()).map(|(&l, &r)| l.abs_diff(r) as i32).sum::<i32>()
}

pub fn part2() -> i32 {
    let input = read_input();
    let (l_vec, r_vec) = parse_input(input);
    
    let mut similarity_score = 0;
    
    let mut r_map: HashMap<i32, i32> = HashMap::new();
    for &item in r_vec.iter() {
        r_map.entry(item).and_modify(|v| *v += 1).or_insert(1);
    }
    
    for item in l_vec.iter() {
        similarity_score += *item * r_map.get(item).unwrap_or(&0);
    }
    
    similarity_score
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