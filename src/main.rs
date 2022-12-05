
pub mod lib;
pub mod day1;


use crate::day1::advent::{count_max_calories, sum_top_three_calories};
use crate::lib::lib::to_lines;

fn main() 
{
    advent_day_1();
}

pub fn advent_day_1()
{
    // part 1
    if let Ok(input_data) = std::fs::read_to_string("./advent_day_1_1_real")
    {
        
        if let Some(max_calory_count) = count_max_calories(to_lines(&input_data))
        {
            println!("Maximum calory count in the elves: {}", max_calory_count);
        }
        else
        {
            println!("No maximum calory count could be found.  Something seems to have gone wrong.");
        }
    }
    else 
    {
        panic!("The file ain't there you dope.")
    }

    // part 2
    if let Ok(input_data) = std::fs::read_to_string("./advent_day_1_1_real")
    {
        let max_three = sum_top_three_calories(to_lines(&input_data));
        println!("Sum of the top three calory counts: {}", max_three);
        
    }
}