
pub mod lib;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;

use day10::advent::execute;
use day11::advent::monkey_business;
use day11::part2::solver;
use day12::advent::solver_day12;
use day13::advent::{solve_day_13, solve_day_13_2};
use day14::advent::{solve_day_14_1, solve_day_14_2};
use day2::advent::evaluate_tournament;
use day3::advent::analyze_rucksacks;
use day4::advent::count_contained_pairs;
use day6::advent::scan_datastream;
use day7::advent::{space_finder};
use day8::advent::part1;
use day9::advent::solve_day_9;
use lib::lib::to_untrimmed_lines;

use crate::day1::advent::{count_max_calories, sum_top_three_calories};
use crate::day2::advent::evaluate_tournament_the_second;
use crate::day3::advent::analyze_badges;
use crate::lib::lib::to_lines;

fn main() 
{
    env_logger::init();
    // advent_day_1();
    // advent_day_2();
    // advent_day_3()
    // advent_day_4();
    // advent_day_5();
    // advent_day_6();
    // advent_day_7();
    // advent_day_8();
    // advent_day_9()
    // advent_day_11();
    // advent_day_12()
    // advent_day_13()
    advent_day_14()
}

pub fn advent_day_14()
{
    let input_data = read_file_to_str("./advent_day_14_real");
    let lines = to_lines(&input_data);

    // solve_day_14_1(lines);
    solve_day_14_2(lines);

}

pub fn advent_day_13()
{
    let input_data = read_file_to_str("./advent_day_13_real");
    let lines = to_lines(&input_data);

    solve_day_13_2(lines);
}

pub fn advent_day_12()
{
    let input_data = read_file_to_str("./advent_day_12_real");
    let lines = to_lines(&input_data);

    solver_day12(lines);
}

pub fn advent_day_11()
{
    let input_data = read_file_to_str("./advent_day_11_real");
    let lines = to_lines(&input_data);

    solver(lines);
}

pub fn advent_day_10()
{
    let input_data = read_file_to_str("./advent_day_10_real");
    let lines = to_lines(&input_data);

    execute(lines);
}

pub fn advent_day_9()
{
    let input_data = read_file_to_str("./advent_day_9_test");
    let lines = to_lines(&input_data);

    solve_day_9(lines);
}

pub fn advent_day_8()
{
    let input_data = read_file_to_str("./advent_day_8_real");
    let lines = to_lines(&input_data);

    part1(lines)
}

pub fn advent_day_7()
{
    let input_data = read_file_to_str("./advent_day_7_1_real");
    let lines = to_lines(&input_data);

    // dir_solver(lines);
    space_finder(lines);
}

pub fn advent_day_6()
{
    let input_data = read_file_to_str("./advent_day_6_1_real");
    let end_of_marker = scan_datastream(input_data.trim());

    println!("End of marker: {}", end_of_marker);
}

pub fn advent_day_5()
{
    let input_data = read_file_to_str("./advent_day_5_1_real");
    let lines = to_untrimmed_lines(&input_data);

    println!("last state of crates: {}", day5::advent::solver(lines));
}

pub fn advent_day_4()
{
    let input_data = read_file_to_str("./advent_day_4_1_test");
    let lines = to_lines(&input_data);

    let (contained_count, overlap_count) = count_contained_pairs(lines);

    println!("The total number of wholly contained pairs is {}, and the number of overlaps is {}", contained_count, overlap_count);
}

pub fn advent_day_3()
{
    
    let input_data = read_file_to_str("./advent_day_3_1_real");
    let lines = to_lines(&input_data);
    let priority = analyze_rucksacks(&lines);

    let badge_priority = analyze_badges(&lines);

    println!("The total priority of the dataset is {}", priority);
    println!("The badge priority is: {}", badge_priority);
}

fn read_file_to_str(path: &str) -> String
{
    if let Ok(input_data) = std::fs::read_to_string(path)
    {
        return input_data;
    }
    else
    {
        panic!("There is no file here named {}, you goon.", path);
    }
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