
pub fn count_contained_pairs(pairs: Vec<&str>) -> (u64, u64)
{
    let mut fully_contained: u64 = 0;
    let mut any_overlap: u64 = 0;

    for pair_str in pairs
    {
        if pair_str.is_empty()
        {
            continue;
        }

        let (elf_1_range, elf_2_range) = construct_range_pair(pair_str);
        if contains(&elf_1_range, &elf_2_range) || contains(&elf_2_range, &elf_1_range)
        {
            fully_contained += 1;
        }

        if overlaps(&elf_1_range, &elf_2_range)
        {
            any_overlap += 1;
        }
    }

    return (fully_contained, any_overlap);
}

pub fn construct_range_pair(range_str: &str) -> (SectionRange, SectionRange)
{
    let mut ranges = range_str.split(",");

    let (elf_1_range, elf_2_range) = (ranges.next().unwrap(), ranges.next().unwrap());

    let range_vec_1 = get_start_end(&elf_1_range.split("-").collect::<Vec<&str>>());
    let range_vec_2 = get_start_end(&elf_2_range.split("-").collect::<Vec<&str>>());
    
    (SectionRange::new(range_vec_1.0, range_vec_1.1), SectionRange::new(range_vec_2.0, range_vec_2.1))
}

fn get_start_end(range_vec: &Vec<&str>) -> (u16, u16)
{
    if range_vec.len() == 0
    {
        panic!("There is no range in the vector.  This is the bads.");
    }

    if let Ok(start) = u16::from_str_radix(range_vec.first().unwrap(), 10)
    {
        if let Ok(end) = u16::from_str_radix(range_vec.last().unwrap(), 10)
        {
            return (start, end);
        }
    }
    
    panic!("One or both of the range_vec values was non-numeric.");
}

pub fn contains(container: &SectionRange, containee: &SectionRange) -> bool
{
    container.start <= containee.start && container.end >= containee.end
}

pub fn overlaps(range_1: &SectionRange, range_2: &SectionRange) -> bool
{
    (range_1.start <= range_2.start && range_1.end >= range_2.start) || 
    (range_2.start <= range_1.start && range_2.end >= range_1.start)
}

pub struct SectionRange
{
    pub start: u16,
    pub end: u16,
}

impl SectionRange {
    
    pub fn new(start: u16, end: u16) -> SectionRange { SectionRange{start, end}}

}

#[cfg(test)]
pub mod tests
{
    use crate::day4::advent::{SectionRange, contains, overlaps};

    use super::construct_range_pair;

    #[test]
    pub fn overlaps_returns_false_on_disjoint_ranges_where_range_1_ends_before_range_2()
    {
        let disjoint_range_1 = SectionRange::new(1,4);
        let disjoint_range_2 = SectionRange::new(5,25);

        assert!(!overlaps(&disjoint_range_1, &disjoint_range_2));
    }

    #[test]
    pub fn overlaps_returns_false_on_disjoint_ranges_where_range_2_ends_before_range_1()
    {
        let disjoint_range_1 = SectionRange::new(1,4);
        let disjoint_range_2 = SectionRange::new(5,25);

        assert!(!overlaps(&disjoint_range_2, &disjoint_range_1));
    }

    #[test]
    pub fn overlaps_returns_true_when_ranges_share_at_least_one_element()
    {
        let intersecting_range_1 = SectionRange::new(3, 8);
        let intersecting_range_2 = SectionRange::new(8, 12);

        assert!(overlaps(&intersecting_range_1, &intersecting_range_2));
    }

    #[test]
    pub fn overlaps_returns_true_when_ranges_share_at_least_one_element_and_range_2_starts_first()
    {
        let intersecting_range_1 = SectionRange::new(3, 8);
        let intersecting_range_2 = SectionRange::new(8, 12);

        assert!(overlaps(&intersecting_range_2, &intersecting_range_1));
    }

    #[test]
    pub fn overlaps_returns_true_when_one_range_wholly_contains_another()
    {
        let container = SectionRange::new(1, 12);
        let containee = SectionRange::new(3, 9);

        assert!(overlaps(&container, &containee));
        assert!(overlaps(&containee, &container));
    }

    #[test]
    pub fn construct_range_pair_generates_section_range_with_ranges_of_size_1_when_no_range_separator_is_in_input()
    {
        let range_str = "4,5";

        let (range_a, range_b) = construct_range_pair(range_str);

        assert_eq!(range_a.start, range_a.end);
        assert_eq!(range_b.start, range_b.end);
    }

    #[test]
    pub fn construct_range_pair_generates_section_range_with_non_unit_ranges_when_range_separator_is_in_input()
    {
        let range_str = "3-4,4-5";
        
        let (range_a, range_b) = construct_range_pair(range_str);

        assert_ne!(range_a.start, range_a.end);
        assert_ne!(range_b.start, range_b.end);
        assert_eq!(range_a.start, 3);
        assert_eq!(range_a.end, 4);
        assert_eq!(range_b.start, 4);
        assert_eq!(range_b.end, 5);
    }

    #[test]
    pub fn contains_returns_false_for_wholly_disjoint_ranges()
    {
        let disjoint_range_1 = SectionRange::new(1,4);
        let disjoint_range_2 = SectionRange::new(5,25);

        assert!(!contains(&disjoint_range_1, &disjoint_range_2));
        assert!(!contains(&disjoint_range_2, &disjoint_range_1));
    }

    #[test]
    pub fn contains_returns_false_for_partial_overlapping_ranges()
    {
        let disjoint_range_1 = SectionRange::new(1,8);
        let disjoint_range_2 = SectionRange::new(5, 15);

        assert!(!contains(&disjoint_range_1, &disjoint_range_2));
        assert!(!contains(&disjoint_range_2, &disjoint_range_1));
    }

    #[test]
    pub fn contains_returns_true_if_all_of_second_range_is_contained_within_first_range()
    {
        let intersecting_range_1 = SectionRange::new(1,12);
        let intersecting_range_2 = SectionRange::new(4,10);

        assert!(!contains(&intersecting_range_2, &intersecting_range_1));
        assert!(contains(&intersecting_range_1, &intersecting_range_2));
    }
}