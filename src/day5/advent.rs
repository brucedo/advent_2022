use std::{cell::RefCell, borrow::Borrow};

use log::debug;


pub fn solver(lines: Vec<&str>) -> String
{
    let (mut setup, mut operations) = input_processor(lines);

    // setup's top line is just the numbers at the base of every stack.  pop it and use it to initialize the cargo stack.
    let stacks_width = stack_width(setup.pop().unwrap());
    let cargo_stacks = RefCell::new(Vec::<Vec<&str>>::new());

    {
        let mut mut_cargo_stacks = cargo_stacks.borrow_mut();
        for _i in 0..stacks_width
        {
            mut_cargo_stacks.push(Vec::<&str>::new())
        }
    }

    while !setup.is_empty() {stackifier(&cargo_stacks, columnizer(setup.pop().unwrap()))}

    while !operations.is_empty()
    {
        let operation = operations.remove(0);
        let (count, from, to) = translate_operation(operation);
        move_crates(&cargo_stacks, count, from, to);
    }

    return get_final_state(&cargo_stacks);
}

fn get_final_state(cargo_stacks: &RefCell<Vec<Vec<&str>>>) -> String
{
    let k = cargo_stacks.borrow();
    let mut partial = String::from("");
    for stack in k.as_slice()
    {
        partial.push_str(*stack.get(stack.len() - 1).unwrap());
    }

    return partial;
}

fn move_crates(cargo_stacks: &RefCell<Vec<Vec<&str>>>, count: u64, from: usize, to: usize)
{
    {
        debug!("Cargo stacks is {} stacks wide.", cargo_stacks.borrow().len());
        debug!("Attempting to add {} crate(s) to {}, from {}", count, to, from);
    }
    let mut mut_cargo_stacks = cargo_stacks.borrow_mut();

    let cargo_from = mut_cargo_stacks.get_mut(from).unwrap();
    let mut temp = Vec::<&str>::new();
    for _i in 0..count
    {
        temp.push(cargo_from.pop().unwrap());
        debug!("Removed crate {} from the stack.", temp.get(temp.len() - 1).unwrap());
    }


    let cargo_to = mut_cargo_stacks.get_mut(to).unwrap();
    while !temp.is_empty()
    {
        // cargo_to.push(temp.remove(0));
        cargo_to.push(temp.pop().unwrap());
    }
    
}

fn stack_width(counter: &str) -> u64
{
    let counter_segs = counter.trim().split(" ");
    let last_num = counter_segs.last().unwrap();

    if let Ok(width) = u64::from_str_radix(last_num, 10)
    {
        return width;
    }
    else
    {
        panic!("The value {} should have been numeric but it is not.", last_num);
    }
}

fn translate_operation(operation: &str) -> (u64, usize, usize)
{
    let mut op_tokens = operation.split(" ");
    op_tokens.next(); // move
    let count = u64::from_str_radix(op_tokens.next().unwrap(), 10).unwrap();
    op_tokens.next(); // from
    let from = usize::from_str_radix(op_tokens.next().unwrap(), 10).unwrap() - 1; 
    op_tokens.next(); // to
    let to = usize::from_str_radix(op_tokens.next().unwrap(), 10).unwrap() - 1;

    return (count, from, to);
}

pub fn input_processor(lines: Vec<&str>) -> (Vec<&str>, Vec<&str>)
{
    let mut spliterator = lines.split(|line| line.is_empty());

    let mut setup = Vec::<&str>::new();
    let mut commands = Vec::<&str>::new();

    if let Some(setup_lines) = spliterator.next()
    {
        for line in setup_lines
        {
            setup.push(*line)
        }
    }
    if let Some(command_lines) = spliterator.next()
    {
        for line in command_lines
        {
            commands.push(*line);
        }
    }

    return (setup, commands);
}

pub fn stackifier<'a>(stack_set: &'a RefCell<Vec<Vec<&'a str>>>, new_row: Vec<&'a str>)
{
    let mut mut_stack_set = stack_set.borrow_mut();
    if mut_stack_set.len() != new_row.len()
    {
        panic!("There should be as many new columns in new_row as there are stacks in stack_set.");
    }

    debug!("Adding row: {:?}", new_row);

    for i in 0..new_row.len()
    {
        let new_crate = *new_row.get(i).unwrap();

        debug!("Processing crate type {}", new_crate);
        
        if new_crate.trim().is_empty()
        {
            continue;
        }

        debug!("Crate type is not empty.");

        let stack = mut_stack_set.get_mut(i).unwrap();

        if let Some(entry) = stack.last()
        {
            if entry.trim().is_empty()
            {
                panic!("The input is adding a crate above an empty space which is not valid input.");    
            }
        }
        // if stack.last().unwrap().trim().is_empty()
        // {
        //     panic!("The input is adding a crate above an empty space which is not valid input.");
        // }

        stack.push(new_crate);

    }
}

pub fn columnizer(row_str: &str) -> Vec<&str>
{
    let mut column_entries = Vec::new();

    let mut head: &str;
    let mut tail: &str = row_str;

    while tail.len() > 3
    {
        (head, tail) = tail.split_at(4);
        debug!("Head: -- {} --", head);
        debug!("Tail: -- {} --", tail);
        if head.trim().is_empty()
        {
            column_entries.push("");
        }
        else
        {
            column_entries.push(head.strip_prefix("[").unwrap().strip_suffix("] ").unwrap().trim());
        }
    }

    if tail.len() != 3
    {
        panic!("Should be three characters left at the end of this string.");
    }

    if tail.trim().is_empty()
    {
        column_entries.push("");
    }
    else
    {
        column_entries.push(tail.strip_prefix("[").unwrap().strip_suffix("]").unwrap());
    }
    

    return column_entries;
}

#[cfg(test)]
pub mod tests
{
    use std::cell;

    use crate::day5::advent::{columnizer, stackifier, input_processor};

    use super::{stack_width, translate_operation};

    pub fn init()
    {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    pub fn given_a_proper_format_translate_operation_returns_a_triple_of_nums()
    {
        let string = "move 1 to 2 from 3";

        let result = translate_operation(string);

        assert_eq!(result, (1, 2, 3));
    }

    #[test]
    pub fn given_a_space_separated_sequence_of_nums_stack_width_should_give_us_last_num()
    {
        let input = "   1     2   3 4  5    6   7  8    9          ";

        assert_eq!(stack_width(input), 9);
    }

    #[test]
    pub fn given_a_vec_of_only_setup_lines_input_processor_saves_all_lines_to_first_tuple_element()
    {
        let input = vec!["        [F] [G]", "    [D] [E] [I]", "[A] [B] [C] [H]", " 1   2   3   4 ", ""];

        let (setup, commands) = input_processor(input);
        assert_eq!(commands.len(), 0);
        assert_eq!(setup.len(), 4);
        assert_eq!(setup.get(3).unwrap(), &" 1   2   3   4 ");
    }

    #[test]
    pub fn given_a_vec_of_only_commands_input_processor_saves_all_lines_to_second_tuple_element()
    {
        let input = vec!["", "move 1 from 2 to 3", "move 2 from 3 to 4", "move 3 from 4 to 1" ];
        
        let (setup, commands) = input_processor(input);

        assert_eq!(commands.len(), 3);
        assert_eq!(setup.len(), 0);
        assert_eq!(commands.get(2).unwrap(), &"move 3 from 4 to 1");
    }

    #[test]
    pub fn given_a_vec_of_commands_and_setup_input_processor_places_setup_in_first_tuple_element_and_commands_in_second()
    {
        let input = vec!["[A] [B] [C] [D]", "", "move 1 from 3 to 4"];

        let (setup, commands) = input_processor(input);

        assert_eq!(commands.len(), 1);
        assert_eq!(setup.len(), 1);
        assert_eq!(commands.get(0).unwrap(), &"move 1 from 3 to 4");
        assert_eq!(setup.get(0).unwrap(), &"[A] [B] [C] [D]");
    }

    #[test]
    pub fn given_a_fully_empty_vec_of_crates_stackifier_adds_no_crates_to_any_stack()
    {
        let stacks = cell::RefCell::new(Vec::<Vec<&str>>::new());
        let mut new_row = Vec::<&str>::new();
        new_row.push("");
        new_row.push("");
        new_row.push("");

        {
            let mut mut_stacks = stacks.borrow_mut();
            let mut column1 = Vec::<&str>::new();
            column1.push("D");
            mut_stacks.push(column1);

            let mut column2 = Vec::<&str>::new();
            column2.push("");
            mut_stacks.push(column2);

            let mut column3 = Vec::<&str>::new();
            column3.push("A");
            mut_stacks.push(column3);

            
        }
        stackifier(&stacks, new_row);

        let immut_stacks = stacks.borrow();

        assert_eq!(immut_stacks.len(), 3);
        assert_eq!(immut_stacks.get(0).unwrap().len(), 1);
        assert_eq!(immut_stacks.get(1).unwrap().len(), 1);
        assert_eq!(immut_stacks.get(2).unwrap().len(), 1);
    }

    #[test]
    pub fn given_a_partially_filled_vec_of_crates_stackifier_adds_only_to_stack_where_new_crates_exist()
    {
        let stacks = cell::RefCell::new(Vec::<Vec<&str>>::new());

        {
            let mut mut_stacks = stacks.borrow_mut();
            let mut column1 = Vec::<&str>::new();
            column1.push("D");
            mut_stacks.push(column1);

            let mut column2 = Vec::<&str>::new();
            column2.push("");
            mut_stacks.push(column2);

            let mut column3 = Vec::<&str>::new();
            column3.push("A");
            mut_stacks.push(column3);
        }
        
        let mut new_row = Vec::<&str>::new();
        new_row.push("");
        new_row.push("");
        new_row.push("F");

        stackifier(&stacks, new_row);
        
        assert_eq!(stacks.borrow().len(), 3);
        assert_eq!(stacks.borrow().get(0).unwrap().len(), 1);
        assert_eq!(stacks.borrow().get(1).unwrap().len(), 1);
        assert_eq!(stacks.borrow().get(2).unwrap().len(), 2);
        assert_eq!(stacks.borrow().get(2).unwrap().get(1).unwrap(), &"F");
    }

    #[test]
    pub fn given_an_empty_str_columnizer_produces_vec_of_empty_strings_of_length_equal_to_number_of_columns_repr_by_str()
    {
        init();
        //                      --- --- --- --- --- --- --- --- 8 empty columns represented
        let column_str = "                               ";
        let columns = columnizer(column_str);

        assert_eq!(columns.len(), 8);
        for column in columns
        {
            assert!(column.is_empty());
        }
    }

    #[test]
    pub fn given_a_str_with_one_or_more_typed_columns_columnizer_produces_vec_whose_corresponding_elements_are_matching_type()
    {
        init();
        //                      --- --- --- --- --- --- --- 4th and 7th non-empty respectively
        let column_str = "            [D]         [E]";
        let columns = columnizer(column_str);

        assert_eq!(columns.len(), 7);
        assert!(columns.get(3).is_some());
        assert_eq!(columns.get(3).unwrap(), &"D");
        assert!(columns.get(6).is_some());
        assert_eq!(columns.get(6).unwrap(), &"E");

    }
}