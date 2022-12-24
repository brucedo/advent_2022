
use std::{collections::HashMap, cmp::Ordering};


pub fn solve_day_13(lines: Vec<&str>)
{
    let pairs = break_lines_into_pairs(lines);

    let mut index: usize = 1;
    let mut fails: usize = 0;
    for (left_str, right_str) in pairs
    {
        let mut left = build_list(left_str);
        let mut right = build_list(right_str);

        if compare_lists(left, right)
        {
            println!("List pair index {} failed the comparison test.", index);
            fails += index;
        }

        index += 1;
    }

    println!("Sum of all failed indices: {}", fails);
}

fn break_lines_into_pairs(lines: Vec<&str>) -> Vec<(&str, &str)>
{
    let mut pairs = Vec::<(&str, &str)>::new();

    let mut left: &str = "";

    let mut count: usize = 0;

    for line in lines
    {
        if line.trim().is_empty()
        {
            continue;
        }

        if count % 2 == 0
        {
            left = line;
        }
        else
        {
            pairs.push((left, line));
        }

        count += 1;
    }

    return pairs;
}

#[derive(PartialEq)]
pub struct List
{
    lists: HashMap<usize, List>,
    scalars: HashMap<usize, i32>,
    next_index: usize,
    front_index: usize,
}

impl List
{
    pub fn new() -> List
    {
        List
        {
            lists: HashMap::new(),
            scalars: HashMap::new(),
            next_index: 0,
            front_index: 0,
        }
    }

    pub fn push_back_scalar(&mut self, scalar: i32)
    {
        self.scalars.insert(self.next_index, scalar);
        self.next_index += 1;
    }

    pub fn push_back_list(&mut self, list: List)
    {
        self.lists.insert(self.next_index, list);
        self.next_index += 1;
    }

    pub fn pop_front(&mut self) -> Type
    {
        if self.front_index >= self.next_index { return Type::None; }

        if self.lists.contains_key(&self.front_index)
        {
            let value = self.lists.remove(&self.front_index).unwrap();
            self.front_index += 1;
            return Type::List(value);
        }
        else if self.scalars.contains_key(&self.front_index)
        {
            let value = self.scalars.remove(&self.front_index).unwrap();
            self.front_index += 1;
            return Type::Scalar(value);
        }

        return Type::None;
    }

    pub fn len(&self) -> usize
    {
        self.lists.len() + self.scalars.len()
    }

    pub fn remove(&mut self, index: usize) -> Type
    {
        if self.lists.contains_key(&index)
        {
            let value = self.lists.remove(&index).unwrap();
            return Type::List(value);
        }
        else if self.scalars.contains_key(&index)
        {
            let value = self.scalars.remove(&index).unwrap();
            return Type::Scalar(value);
        }
        else
        {
            return Type::None;
        }
    }

}

pub fn promote_to_list(int_value: i32) -> List
{
    let mut container = List::new();
    container.push_back_scalar(int_value);

    return container;
}

pub fn compare_lists(left: List, right: List) -> bool
{
    
    match recursive_compare_lists(left, right)
    {
        Ordering::Less => 
        {
            true
        }
        _ => {false}
    }

}

fn recursive_compare_lists(mut left: List, mut right: List) -> Ordering
{
    loop
    {

        match (left.pop_front(), right.pop_front())
        {
            (Type::List(left_list), Type::List(right_list)) => 
            {
                match recursive_compare_lists(left_list, right_list)
                {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                    _ => {}
                }
            },
            (Type::Scalar(left_scalar), Type::Scalar(right_scalar)) =>
            {
                if left_scalar < right_scalar { return Ordering::Less}
                else if left_scalar > right_scalar { return Ordering::Greater;}
            },
            (Type::None, Type::None) => 
            {
                return Ordering::Equal;
            },
            (Type::None, Type::List(_) | Type::Scalar(_)) => 
            {
                return Ordering::Less;
            },
            (Type::List(_) | Type::Scalar(_), Type::None) =>
            {
                return Ordering::Greater;
            },
            (Type::List(left_list), Type::Scalar(right_scalar)) =>
            {
                let right_list = promote_to_list(right_scalar);
                match recursive_compare_lists(left_list, right_list)
                {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                    _ => {}
                }
            },
            (Type::Scalar(left_scalar), Type::List(right_list)) =>
            {
                let left_list = promote_to_list(left_scalar);
                match recursive_compare_lists(left_list, right_list)
                {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                    _ => {}
                }
            }
        }
    }
}

pub fn build_list(list_str: &str) -> List
{
    if !list_str.starts_with('[')
    {
        panic!("Invalid input string.")
    }

    let chars = list_str[1..list_str.len()].chars();
    let mut list: List = List::new();
    let mut list_stack = Vec::<List>::new();
    let mut completed_scalar: Option<i32> = None;
    let mut under_construction:i32 = 0;

    for char in chars
    {
        if char == '['
        {
            list_stack.push(list);
            list = List::new();
        }
        else if char == ']'
        {

            if completed_scalar.is_some()
            {
                list.push_back_scalar(completed_scalar.unwrap());
                under_construction = 0;
                completed_scalar = None;
            };

            if let Some(mut parent_list) = list_stack.pop()
            {
                parent_list.push_back_list(list);
                list = parent_list;
            }
            else
            {
                break;
            }
        }
        else
        {
            
            if char.is_numeric()
            {
                under_construction *= 10;
                under_construction += char.to_digit(10).unwrap() as i32;
                completed_scalar = Some(under_construction);
            }
            else
            {
                if completed_scalar.is_some()
                {
                    list.push_back_scalar(completed_scalar.unwrap());
                    under_construction = 0;
                    completed_scalar = None;
                };
            }
        }
    }

    return list;
}

#[derive(PartialEq)]
pub enum Type
{
    Scalar(i32),
    List(List),
    None,
}

impl Type 
{
    pub fn is_some(&self) -> bool
    {
        self != &Type::None
    }

    pub fn is_list(&self) -> bool
    {
        match self 
        {
            Type::None => { false },
            Type::List(_) => { true },
            Type::Scalar(_) => { false }
        }
    }

    pub fn is_scalar(&self) -> bool
    {
        match self 
        {
            Type::None => { false },
            Type::List(_) => { false },
            Type::Scalar(_) => { true }
        }
    }
}


#[cfg(test)]
pub mod tests
{
    use core::panic;

    use crate::day13::advent::Type;
    use crate::day13::advent::List;

    use super::compare_lists;
    use super::promote_to_list;
    use super::build_list;

    #[test]
    pub fn empty_child_lists_are_still_items_so_if_right_runs_out_of_empty_lists_first_comparison_must_fail_and_if_left_runs_out_comparison_must_succeed()
    {
        let left_str = "[[[]]]";
        let right_str = "[[]]";

        let left = build_list(left_str);
        let right = build_list(right_str);

        assert!(!compare_lists(left, right));

        let right_str = "[[[]]]";
        let left_str = "[[]]";

        let left = build_list(left_str);
        let right = build_list(right_str);

        assert!(compare_lists(left, right));

    }

    #[test]
    pub fn compare_list_will_promote_a_scalar_to_a_list_if_types_mismatch_and_compare_promoted_list_to_original_list()
    {
        let left_str = "[[1],[2,3,4]]";
        let right_str = "[[1],4]";

        let left = build_list(left_str);
        let right = build_list(right_str);

        assert!(compare_lists(left, right));

        let left_str = "[9]";
        let right_str = "[[8,7,6]]";

        let left = build_list(left_str);
        let right = build_list(right_str);

        assert!(!compare_lists(left, right))
    }

    #[test]
    pub fn compare_list_returns_true_if_left_is_shorter_and_all_elements_of_left_equal_right()
    {
        let left_str = "[]";
        let right_str = "[3]";

        let left = build_list(left_str);
        let right = build_list(right_str);

        assert!(compare_lists(left, right));
    }

    #[test]
    pub fn compare_list_returns_false_if_right_is_shorter_and_all_elements_of_left_equal_right()
    {
        let right_str = "[7,7,7]";
        let left_str = "[7,7,7,7]";

        let left = build_list(left_str);
        let right = build_list(right_str);

        assert!(!compare_lists(left, right));
    }

    #[test]
    pub fn compare_list_returns_true_for_simple_scalar_only_left_and_right_if_left_has_no_integers_larger_than_right_and_at_least_one_less()
    {
        let left_str = "[1,1,3,1,1]";
        let right_str = "[1,1,5,1,1]";

        let left = build_list(left_str);
        let right = build_list(right_str);

        assert!(compare_lists(left, right));
    }

    #[test]
    pub fn compare_list_returns_false_for_simple_scalar_only_lists_if_left_has_at_least_one_integer_larger_than_right()
    {
        let left_str = "[1,1,5,1,1]";
        let right_str = "[1,1,3,1,1]";

        let left = build_list(left_str);
        let right = build_list(right_str);

        assert!(!compare_lists(left, right));
    }

    #[test]
    pub fn when_handed_a_string_with_an_empty_list_build_list_produces_a_list_with_no_elements()
    {
        let list_str = "[]";
        let list: List = build_list(list_str);

        assert_eq!(list.len(), 0);
    }

    #[test]
    pub fn when_handed_a_string_with_a_nested_list_build_list_produces_a_mixed_list()
    {
        let list_str = "[[1],4]";
        let mut list: List = build_list(list_str);

        assert_eq!(list.len(), 2);
        match list.pop_front() 
        {
            Type::List(mut sublist) => 
            {
                assert_eq!(sublist.len(), 1);
                match sublist.pop_front()
                {
                    Type::Scalar(scalar) => {assert_eq!(scalar, 1)}
                    _ => { panic!("Wrong return type: should be a scalar 1"); }
                }
            }
            _ => { panic!("Wrong return type: should be a nested list."); }
        }
        match list.pop_front()
        {
            Type::Scalar(scalar) => {assert_eq!(scalar, 4);}
            _ => { panic!("Wrong return type: should be a scalar."); }
        }
    }

    #[test]
    pub fn multi_digit_scalars_are_captured_at_any_list_level_by_build_list()
    {
        let list_str = "[[23], 45]";
        let mut list = build_list(list_str);
        
        assert_eq!(list.len(), 2);

        match list.pop_front() 
        {
            Type::List(mut sublist) => 
            {
                assert_eq!(sublist.len(), 1);
                match sublist.pop_front()
                {
                    Type::Scalar(scalar) => {assert_eq!(scalar, 23)}
                    _ => { panic!("Wrong return type: should be a scalar 1"); }
                }
            }
            _ => { panic!("Wrong return type: should be a nested list."); }
        }
        match list.pop_front()
        {
            Type::Scalar(scalar) => {assert_eq!(scalar, 45);}
            _ => { panic!("Wrong return type: should be a scalar."); }
        }
    }

    #[test]
    pub fn when_handed_a_string_with_no_nested_lists_build_list_produces_a_list_of_just_scalars()
    {
        let list_str = "[1,1,3,1,1]";
        
        let mut list:List = build_list(list_str);

        assert_eq!(list.len(), 5);
        match list.pop_front() 
        {
            Type::Scalar(scalar) => {assert_eq!(scalar, 1);}
            _ => {panic!("Wrong type returned!");}
        }
        match list.pop_front() 
        {
            Type::Scalar(scalar) => {assert_eq!(scalar, 1);}
            _ => {panic!("Wrong type returned!");}
        }
        match list.pop_front() 
        {
            Type::Scalar(scalar) => {assert_eq!(scalar, 3);}
            _ => {panic!("Wrong type returned!");}
        }
        match list.pop_front() 
        {
            Type::Scalar(scalar) => {assert_eq!(scalar, 1);}
            _ => {panic!("Wrong type returned!");}
        }
        match list.pop_front() 
        {
            Type::Scalar(scalar) => {assert_eq!(scalar, 1);}
            _ => {panic!("Wrong type returned!");}
        }
        match list.pop_front() 
        {
            Type::None => {}
            _ => {panic!("Wrong type returned!");}
        }
    }

    #[test]
    pub fn a_list_will_return_the_oldest_inserted_item_when_pop_front_is_called_and_will_not_retain_a_copy()
    {
        let mut list = List::new();

        let mut list2 = List::new();
        list2.push_back_scalar(22);
        list2.push_back_scalar(14);

        list.push_back_scalar(0);
        list.push_back_list(list2);
        list.push_back_scalar(12);
        list.push_back_scalar(14);

        assert_eq!(list.len(), 4);
        match list.pop_front() 
        {
            Type::Scalar(item1) => { assert_eq!(item1, 0);}
            _ => panic!("Wrong type returned - should have been scalar 0")
        }

        match list.pop_front()
        {
            Type::List(item2) => { assert_eq!(item2.len(), 2)}
            _ => panic!("Wrong type returned - should have been a list of length 2")
        }
    }

    #[test]
    pub fn a_lists_len_function_returns_the_count_of_scalars_plus_lists_in_the_list()
    {
        let mut list = List::new();
        list.push_back_scalar(0);
        list.push_back_scalar(12);
        list.push_back_scalar(14);

        let mut list2 = List::new();
        list2.push_back_scalar(22);
        list2.push_back_scalar(14);

        list.push_back_list(list2);

        assert_eq!(list.len(), 4);
    }

    #[test]
    pub fn passing_an_integer_to_promote_to_list_will_return_a_boxed_list_containing_the_integer()
    {
        let int_val = 16;

        let mut listified = promote_to_list(int_val);

        assert_eq!(listified.len(), 1);
        match listified.remove(0)
        {
            Type::Scalar(item) => {assert_eq!(item, 16)}
            _ => { panic!("Wrong type returned.")}
        }

    }

    
}