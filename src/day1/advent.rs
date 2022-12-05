
use crate::lib::lib::find_max;

pub fn count_max_calories(calory_list: Vec<&str>) -> Option<u64>
{
    let per_elf = parse_into_vec(calory_list);

    if let Some(max) = find_max(&per_elf)
    {
        return Some(*max);
    }

    None
}

pub fn sum_top_three_calories(calory_list: Vec<&str>) -> u64
{
    let mut per_elf = parse_into_vec(calory_list);

    per_elf.sort_unstable();

    let mut highest_three: u64 = 0;
    for _i in 0..3
    {
        if let Some(calories) = per_elf.pop()
        {
            highest_three += calories;
        }
        else
        {
            break;
        }
    }

    return  highest_three;

}

fn parse_into_vec(calory_list: Vec<&str>) -> Vec<u64>
{

    let mut per_elf = Vec::<u64>::new();

    let mut current_elf_total: u64 = 0;
    for calory_line in calory_list
    {
        if calory_line.trim().is_empty()
        {
            per_elf.push(current_elf_total);
            current_elf_total = 0;
            continue;
        }
        
        if let Ok(calories) = u64::from_str_radix(calory_line.trim(), 10)
        {
            current_elf_total += calories;
        }
        else
        {
            panic!("Really should never get an unparseable number here.")
        }
    }

    // messy cleanup
    per_elf.push(current_elf_total);

    return per_elf;

}


#[cfg(test)]

#[test]
pub fn if_input_is_all_newlines_count_calories_produces_zero()
{
    let mut empty_vec = Vec::<&str>::new();
    empty_vec.push("\n");
    empty_vec.push("\r\n");
    empty_vec.push("\n");
    empty_vec.push("\r\n");
    empty_vec.push("");
    empty_vec.push("");

    let max = count_max_calories(empty_vec);
    assert!(max.is_some());
    assert_eq!(max.unwrap(), 0);
}

#[test] 
pub fn if_input_is_only_one_elf_long_then_return_that_elfs_caloric_count()
{
    let mut one_elf_vec = Vec::<&str>::new();

    one_elf_vec.push("1000");
    one_elf_vec.push("2000");
    one_elf_vec.push("3000");

    let max = count_max_calories(one_elf_vec);
    assert!(max.is_some());
    assert_eq!(max.unwrap(), 6000);
}

#[test]
pub fn if_input_is_many_elves_then_return_the_largest_count()
{
    let mut many_elf_vec = Vec::<&str>::new();

    many_elf_vec.push("1000");
    many_elf_vec.push("1000");
    many_elf_vec.push("1000");
    many_elf_vec.push("");
    many_elf_vec.push("3000");
    many_elf_vec.push("3000");
    many_elf_vec.push("3000");
    many_elf_vec.push("");
    many_elf_vec.push("2000");
    many_elf_vec.push("2000");
    many_elf_vec.push("2000");

    let max = count_max_calories(many_elf_vec);
    assert!(max.is_some());
    assert_eq!(max.unwrap(), 9000);
}

#[test]
pub fn if_input_is_empty_sum_of_top_three_returns_zero()
{
    let mut empty_vec = Vec::<&str>::new();
    empty_vec.push("\n");
    empty_vec.push("\r\n");
    empty_vec.push("\n");
    empty_vec.push("\r\n");
    empty_vec.push("");
    empty_vec.push("");

    let top_three = sum_top_three_calories(empty_vec);

    assert_eq!(top_three, 0);
   
}

#[test]
pub fn if_input_has_fewer_than_three_elves_top_three_returns_sum_of_all()
{
    let mut many_elf_vec = Vec::<&str>::new();

    many_elf_vec.push("1000");
    many_elf_vec.push("1000");
    many_elf_vec.push("1000");
    many_elf_vec.push("");
    many_elf_vec.push("3000");
    many_elf_vec.push("3000");
    many_elf_vec.push("3000");
    
    let top_three = sum_top_three_calories(many_elf_vec);

    assert_eq!(top_three, 12000);
}


#[test]
pub fn if_input_has_more_than_three_elves_top_three_returns_only_sum_of_three_highest_calorie_counts()
{
    let mut many_elf_vec = Vec::<&str>::new();

    many_elf_vec.push("1000");
    many_elf_vec.push("1000");
    many_elf_vec.push("1000");
    many_elf_vec.push("");
    many_elf_vec.push("2000");
    many_elf_vec.push("2000");
    many_elf_vec.push("2000");
    many_elf_vec.push("");
    many_elf_vec.push("3000");
    many_elf_vec.push("3000");
    many_elf_vec.push("3000");
    many_elf_vec.push("");
    many_elf_vec.push("4000");
    many_elf_vec.push("4000");
    many_elf_vec.push("4000");
    many_elf_vec.push("");
    many_elf_vec.push("5000");
    many_elf_vec.push("5000");
    many_elf_vec.push("5000");

    let top_three = sum_top_three_calories(many_elf_vec);

    assert_eq!(top_three, 36000);
}