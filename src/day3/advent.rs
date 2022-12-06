use std::collections::HashSet;

use log::debug;

pub fn analyze_badges(sacks: &Vec<&str>) -> u64
{
    let mut priority: u64 = 0;



    for i in (0..sacks.len()).step_by(3)
    {
        // Stupid hack but why waste time.
        let group_a = sacks.get(i).unwrap();
        if group_a.len() == 0
        {
            continue;
        }
        
        if i + 2 >= sacks.len()
        {
            panic!("The number of sacks is not divisible by 3.");
        }
        
        let group_b = sacks.get(i+1).unwrap();
        let group_c = sacks.get(i + 2).unwrap();

        let ab_overlap = analyze_compartments(group_a, group_b);
        let bc_overlap = analyze_compartments(group_b, group_c);

        let badge = find_common_type(ab_overlap, bc_overlap);
        
        priority += char_to_num(&badge) as u64;
    }

    return  priority;
}

fn find_common_type(ab_overlap: Vec<char>, bc_overlap: Vec<char>) -> char
{
    let mut ab_set = HashSet::<char>::new();

    for overlap in ab_overlap
    {
        ab_set.insert(overlap);
    }

    for overlap in bc_overlap
    {
        if ab_set.contains(&overlap)
        {
            return overlap;
        }
    }

    panic!("No overlap between AB and BC.  You trixxed me!");
}

pub fn analyze_rucksacks(sacks: &Vec<&str> ) -> u64
{
    let mut running_total: u64 = 0;

    for sack in sacks
    {
        if sack.len() == 0
        {
            debug!("Skipping a row.");
            continue;
        }

        let split_point = sack.len() / 2;
        debug!("Split point for this line: {}", split_point);
        let (left, right) = sack.split_at(split_point);
        debug!("Left and right: {}/{}", left, right);

        let overlaps = analyze_compartments(left, right);
        debug!("Overlapping characters: {:?}", overlaps);
        let priorities = transform_priorities(&overlaps);
        running_total += priorities.iter().sum::<u64>();
    }

    return running_total;
}

pub fn analyze_compartments<'a>(left: &'a str, right: &'a str) -> Vec<char>
{
    let mut overlaps = Vec::new();

    let mut left_set = HashSet::<char>::new();
    let mut overlap_set = HashSet::<char>::new();
    
    for char in left.chars()
    {
        left_set.insert(char);
    }

    for char in right.chars()
    {
        
        if left_set.contains(&char)
        {
            overlap_set.insert(char);
        }
    }

    for overlapping in overlap_set.drain() {overlaps.push(overlapping);}

    return overlaps;
}

pub fn transform_priorities(overlaps: &Vec<char>) -> Vec<u64>
{
    let mut priorities = Vec::new();

    for overlap in overlaps
    {
        priorities.push(char_to_num(overlap) as u64);
    }

    return priorities;
}

fn char_to_num(src: &char) -> u8
{
    // sigh
    match src
    {
        'a' => 1, 
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => {panic!("There should only be letters a-zA-Z here.")}
    }
}

#[cfg(test)]
pub mod tests
{
    use crate::day3::advent::{analyze_compartments, transform_priorities};

    #[test]
    pub fn transform_priorities_returns_empty_vec_when_given_empty_vec()
    {
        let overlaps = Vec::<char>::new();

        assert_eq!(transform_priorities(&overlaps).len(), 0);
    }

    #[test]
    pub fn transform_priorities_returns_two_elements_1_and_52_when_given_a_and_uppercase_z_chars()
    {
        let overlaps = vec!['a', 'Z'];

        assert_eq!(transform_priorities(&overlaps).len(), 2);
        assert_eq!(transform_priorities(&overlaps).get(0).unwrap(), &1u64);
        assert_eq!(transform_priorities(&overlaps).get(1).unwrap(), &52u64);
    }
    

    #[test]
    pub fn when_analyze_compartments_receives_two_empty_sets_no_common_types_are_found()
    {
        let left = "";
        let right = "";

        let common_types: Vec<char> = analyze_compartments(left, right);
        assert!(common_types.len() == 0);
    }

    #[test]
    pub fn when_analyze_components_receives_one_empty_set_no_common_types_are_found()
    {
        let left = "abcdefg";
        let right = "ABCDEFG";
        let empty_left = "";
        let empty_right = "";

        assert_eq!(analyze_compartments(left, empty_right).len(), 0);
        assert_eq!(analyze_compartments(empty_left, right).len(), 0);
    }

    #[test]
    pub fn when_analyze_compartments_receives_two_disjoint_sets_no_common_types_are_found()
    {
        let left = "abcdefg";
        let right = "hijklmn";

        assert_eq!(analyze_compartments(left, right).len(), 0);
    }

    #[test]
    pub fn when_analyze_compartments_receives_two_intersecting_sets_common_types_will_be_found()
    {
        let left = "AbCdEfG";
        let right = "GfEdCbA";

        assert_eq!(analyze_compartments(left, right).len(), 7);
        assert!(analyze_compartments(left, right).contains(&'A'));
        assert!(analyze_compartments(left, right).contains(&'b'));
        assert!(analyze_compartments(left, right).contains(&'C'));
        assert!(analyze_compartments(left, right).contains(&'d'));
        assert!(analyze_compartments(left, right).contains(&'E'));
        assert!(analyze_compartments(left, right).contains(&'f'));
        assert!(analyze_compartments(left, right).contains(&'G'));
    }

    #[test]
    pub fn analyze_compartments_treats_types_as_case_sensitive()
    {
        let left = "abcdefg";
        let right = "ABCDEFG";
        let overlapping_right = "ABcDEFG";

        assert_eq!(analyze_compartments(left, right).len(), 0);
        assert_eq!(analyze_compartments(left, overlapping_right).len(), 1);
        assert_eq!(analyze_compartments(left, overlapping_right).get(0).unwrap(), &'c');
    }
}