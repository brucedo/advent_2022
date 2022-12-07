use log::debug;

pub fn stackifier(stack_set: &mut Vec<Vec<&str>>, new_row: &Vec<&str>)
{
    if stack_set.len() != new_row.len()
    {
        panic!("There should be as many new columns in new_row as there are stacks in stack_set.");
    }


    for i in 0..new_row.len()
    {
        let new_crate = *new_row.get(i).unwrap();
        
        if new_crate.trim().is_empty()
        {
            continue;
        }

        let stack = stack_set.get(i).unwrap();
        if stack.last()

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
    use crate::day5::advent::{columnizer, stackifier};

    pub fn init()
    {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    pub fn given_a_fully_empty_vec_of_crates_stackifier_adds_no_crates_to_any_stack()
    {
        let mut stacks = Vec::<Vec<&str>>::new();

        let mut column1 = Vec::<&str>::new();
        column1.push("D");
        stacks.push(column1);

        let mut column2 = Vec::<&str>::new();
        column2.push("");
        stacks.push(column2);

        let mut column3 = Vec::<&str>::new();
        column3.push("A");
        stacks.push(column3);

        let mut new_row = Vec::<&str>::new();
        new_row.push("");
        new_row.push("");
        new_row.push("");
        stackifier(&mut stacks, &mut new_row);

        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks.get(0).unwrap().len(), 1);
        assert_eq!(stacks.get(1).unwrap().len(), 1);
        assert_eq!(stacks.get(2).unwrap().len(), 1);
    }

    #[test]
    pub fn given_a_partially_filled_vec_of_crates_stackifier_adds_only_to_stack_where_new_crates_exist()
    {
        let mut stacks = Vec::<Vec<&str>>::new();

        let mut column1 = Vec::<&str>::new();
        column1.push("D");
        stacks.push(column1);

        let mut column2 = Vec::<&str>::new();
        column2.push("");
        stacks.push(column2);

        let mut column3 = Vec::<&str>::new();
        column3.push("A");
        stacks.push(column3);
        
        let mut new_row = Vec::<&str>::new();
        new_row.push("");
        new_row.push("");
        new_row.push("F");

        stackifier(&mut stacks, &mut new_row);
        
        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks.get(0).unwrap().len(), 1);
        assert_eq!(stacks.get(1).unwrap().len(), 1);
        assert_eq!(stacks.get(2).unwrap().len(), 2);
        assert_eq!(stacks.get(2).unwrap().get(1).unwrap(), &"F");
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