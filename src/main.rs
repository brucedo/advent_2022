
pub mod lib;
pub mod day1;
pub mod day2;


use day2::advent::evaluate_tournament;

use crate::day1::advent::{count_max_calories, sum_top_three_calories};
use crate::day2::advent::evaluate_tournament_the_second;
use crate::lib::lib::to_lines;

fn main() 
{
    // advent_day_1();
    advent_day_2();
}

pub fn advent_day_2()
{
    if let Ok(input_data) = std::fs::read_to_string("./advent_day_2_1_test")
    {
        let lines = to_lines(&input_data);
        let score = evaluate_tournament(&lines);
        let score_the_second = evaluate_tournament_the_second(&lines);
        println!("Total score: {}", score);
        println!("Revised score: {}", score_the_second);
    }
    else
    {
        panic!("You're missing the file, you dullard.");
    }
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