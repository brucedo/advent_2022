use std::collections::VecDeque;


pub fn monkey_business(lines: Vec<&str>)
{
    let mut monkeys = monkeyfi(lines);

    for i in 0..10000
    {
        for j in 0..monkeys.len()
        {
            let mut throws = Vec::<(usize, i32)>::new();
            {
                let monkey = &mut monkeys[j];
                while let Some(throw) = monkey.inspect_relieve_throw()
                {
                    throws.push(throw);
                }
            }

            for throw in throws
            {
                let monkey = &mut monkeys[throw.0];
                monkey.items.push_back(throw.1);
            }
        }

        // display loop
        println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
        println!("           STATE OF THE MONKEY UNION PASS {}", i);
        for monkey in &monkeys
        {
            println!("Items and worry levels: {:?}", monkey.items);
            println!("Inspection count: {}", monkey.inspect_count);
        }
        println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
    }

    let mut max1: usize = 0;
    let mut max2: usize = 0;

    for monkey in &monkeys
    {
        if monkey.inspect_count >= max1 { max2 = max1; max1 = monkey.inspect_count; }
    }

    println!("Busiest monkeys and total: {} * {} = {}", max1, max2, max1 * max2);
}

pub fn monkeyfi(lines: Vec<&str>) -> Vec<Monkey>
{
    let mut monkeys = Vec::<Monkey>::new();

    let records = recordify(lines);

    for record in records
    {
        monkeys.push(make_monkey(record));
    }

    return monkeys;
}

pub fn make_monkey(record: Vec<&str>) -> Monkey
{

    let mut lines = record.iter();
    
    lines.next(); // Discard the unnecessary Monkey line.
    
    let items = starting_items(lines.next().unwrap());
    let action = make_operation(lines.next().unwrap());
    let test = make_test(lines.next().unwrap());
    let true_target = decode_target(lines.next().unwrap());
    let false_target = decode_target(lines.next().unwrap());

    return Monkey { items, action, test, true_target, false_target, inspect_count: 0 }
}

pub fn decode_target(target_line: &str) -> usize
{
    
    if !(target_line.starts_with("If true: throw to monkey ") || target_line.starts_with("If false: throw to monkey "))
    {
        panic!("Invalid input for target decoder.");
    }

    if target_line.contains("true")
    {
        return usize::from_str_radix(target_line.strip_prefix("If true: throw to monkey ").unwrap(), 10).unwrap();
    }
    else
    {
        return usize::from_str_radix(target_line.strip_prefix("If false: throw to monkey ").unwrap(), 10).unwrap();
    }
}

pub fn make_test(test_line: &str) -> Box<dyn Fn(i32) -> bool>
{
    if !test_line.starts_with("Test:")
    {
        panic!("Bad input line for test builder: {}", test_line);
    }

    let divisor = i32::from_str_radix(test_line.strip_prefix("Test: divisible by ").unwrap(), 10).unwrap();

    return construct_test(divisor);
}

pub fn make_operation(operation_line: &str) -> Box<dyn Fn(i32) -> i32>
{
    if !operation_line.starts_with("Operation:")
    {
        panic!("Bad input line for operation builder: {}", operation_line);
    }

    let tail = operation_line.strip_prefix("Operation:").unwrap();
    let mut tokens = tail.trim().split(" ");
    let (_new, _equals, operand1, operator, operand2) = 
        (tokens.next().unwrap(), tokens.next().unwrap(), tokens.next().unwrap(), tokens.next().unwrap(), tokens.next().unwrap());
    let mut operand: Option<i32> = None;
    let operation: Operation;

    if operand1 != "old"
    {
        operand = Some(i32::from_str_radix(operand1, 10).unwrap());
    }
    else if operand2 != "old"
    {
        operand = Some(i32::from_str_radix(operand2, 10).unwrap());
    }
    
    match operator 
    {
        "+" => {operation = Operation::Add(operand)},
        "-" => {operation = Operation::Subtract(operand)},
        "*" => {operation = Operation::Multiply(operand)},
        "/" => {operation = Operation::Divide(operand)},
        _ => {panic!("Unrecognized operation {}", operator)}
    }

    return construct_operation(operation);
}

pub fn starting_items(start_items_line: &str) -> VecDeque<i32>
{
    let mut items = VecDeque::new();
    if !start_items_line.starts_with("Starting items:")
    {
        panic!("Bad input for starting items builder: {}.", start_items_line);
    }

    let tail = start_items_line.strip_prefix("Starting items:").unwrap();
    for item in tail.split(",")
    {
        if !item.trim().is_empty()
        {
            items.push_back(i32::from_str_radix(item.trim(), 10).unwrap());
        }
    }

    return items;
}

pub fn recordify(lines: Vec<&str>) -> Vec<Vec<&str>>
{
    let mut records = Vec::<Vec<&str>>::new();
    let mut record = Vec::<&str>::new();

    for line in lines
    {
        if line.trim().is_empty()
        {
            records.push(record);
            record = Vec::new();
        }
        else
        {
            record.push(line.trim());
        }
    }

    if !record.is_empty()
    {
        records.push(record);
    }

    return records;
}


pub fn construct_operation(op: Operation) -> Box<dyn Fn(i32) -> i32>
{
    match op 
    {
        Operation::Multiply(opt) => 
        {
            match opt 
            {
                Some(operand2) => {Box::new(move |operand1| operand1 * operand2)}
                None => {Box::new(|operand1| operand1 * operand1)}
            }
        },
        Operation::Divide(opt) => 
        {
            match opt 
            {
                Some(operand2) => {Box::new(move |operand1| operand1 / operand2)}
                None => {Box::new(|operand1| operand1 / operand1)}
            }
        },
        Operation::Subtract(opt) => 
        {
            match opt 
            {
                Some(operand2) => {Box::new(move |operand1| operand1 - operand2)}
                None => {Box::new(|operand1| operand1 - operand1)}
            }
        },
        Operation::Add(opt) => 
        {
            match opt 
            {
                Some(operand2) => {Box::new(move |operand1| operand1 + operand2)}
                None => {Box::new(|operand1| operand1 + operand1)}
            }
        },
    }
}

pub fn construct_test(divisor: i32) -> Box<dyn Fn(i32) -> bool>
{
    Box::new(move |dividend| (dividend % divisor) == 0)
}

pub enum Operation
{
    Multiply(Option<i32>),
    Divide(Option<i32>),
    Subtract(Option<i32>),
    Add(Option<i32>),
}

pub struct Monkey
{
    items: VecDeque<i32>,
    action: Box<dyn Fn(i32) -> i32>,
    test: Box<dyn Fn(i32) -> bool>,
    true_target: usize,
    false_target: usize,
    inspect_count: usize,
}

impl Monkey
{
    pub fn inspect_relieve_throw(&mut self) -> Option<(usize, i32)>
    {
        let next = self.items.pop_front()?;

        let post_worry = self.action.as_ref()(next);
        self.inspect_count += 1;
        let post_relief = post_worry / 1;
        match self.test.as_mut()(post_relief) 
        {
            true => {return Some((self.true_target, post_relief))},
            false => {return Some((self.false_target, post_relief))},
        }
    }
}


#[cfg(test)]
pub mod tests
{
    use std::collections::VecDeque;

    use crate::day11::advent::monkeyfi;

    use super::{Monkey, construct_operation, Operation, construct_test};

    #[test]
    pub fn monkeyfi_will_turn_a_stream_of_monkey_records_into_discrete_monkey_chunks()
    {
        let lines: Vec<&str> = vec!["Monkey 0:",
        "  Starting items: 79, 98",
      "  Operation: new = old * 19", 
      "  Test: divisible by 23", 
      "    If true: throw to monkey 2", 
      "    If false: throw to monkey 3", 
      "", 
      "Monkey 1:", 
      "  Starting items: 54, 65, 75, 74", 
      "  Operation: new = old + 6", 
      "  Test: divisible by 19", 
      "    If true: throw to monkey 2", 
      "    If false: throw to monkey 0", 
      "", 
      "Monkey 2:", 
      "  Starting items: 79, 60, 97", 
      "  Operation: new = old * old", 
      "  Test: divisible by 13", 
      "    If true: throw to monkey 1", 
      "    If false: throw to monkey 3", 
      "", 
      "Monkey 3:", 
      "  Starting items: 74", 
      "  Operation: new = old + 3", 
      "  Test: divisible by 17", 
      "    If true: throw to monkey 0", 
          "If false: throw to monkey 1"];

        let monkeys: Vec<Monkey> = monkeyfi(lines);

        assert_eq!(monkeys.len(), 4);
        assert_eq!(monkeys[0].items, vec![79, 98]);
        assert_eq!(monkeys[0].true_target, 2);
        assert_eq!(monkeys[0].false_target, 3);

        assert_eq!(monkeys.len(), 4);
        assert_eq!(monkeys[3].items, vec![74]);
        assert_eq!(monkeys[3].true_target, 0);
        assert_eq!(monkeys[3].false_target, 1);
    }

    #[test]
    pub fn after_a_monkey_tests_your_worry_level_it_will_remove_the_item_from_its_list_and_return_it_and_the_receiving_monkeys_index_as_a_tuple()
    {
        let mut monkey0 = Monkey
        { 
            items: VecDeque::from(vec![10]), 
            action: construct_operation(Operation::Multiply(Some(9))), 
            test: construct_test(5), 
            true_target: 5, 
            false_target: 3,
            inspect_count: 0
        };

        if let Some((index, worry)) = monkey0.inspect_relieve_throw()
        {
            assert_eq!(index, 5);
            assert_eq!(worry, 30);
        }
        else 
        {
            panic!("inspect_relieve_throw returned None, some was expected.")
        }
    }

    #[test] 
    pub fn a_monkey_returns_the_false_index_if_the_test_does_not_evenly_divide_the_worry_level()
    {
        let mut monkey0 = Monkey
        {
            items: VecDeque::from(vec![3]),
            action: construct_operation(Operation::Add(Some(12))),
            test: construct_test(12),
            true_target: 1,
            false_target: 2,
            inspect_count: 0
        };

        if let Some((index, _worry)) = monkey0.inspect_relieve_throw()
        {
            assert_eq!(index, 2);
        }
        else
        {
            panic!("inspect_relieve_throw returned None, some was expected.")
        }
    }

    #[test]
    pub fn the_relief_divides_worry_by_three_and_takes_the_floor_of_the_result()
    {
        let mut monkey = Monkey
        {
            items: VecDeque::from(vec![5]),
            action: construct_operation(Operation::Add(Some(6))),
            test: construct_test(2),
            true_target: 1,
            false_target: 2,
            inspect_count: 0
        };

        if let Some((_index, worry)) = monkey.inspect_relieve_throw()
        {
            assert_eq!(worry, 3);
        }
        else
        {
            panic!("inspect_relieve_throw returned None, some was expected.")
        }
        
    }

    #[test]
    pub fn if_a_monkey_has_no_items_left_inspect_relieve_throw_returns_none()
    {
        let mut monkey = Monkey
        {
            items: VecDeque::from(vec![]),
            action: construct_operation(Operation::Add(Some(100))),
            test: construct_test(12),
            true_target: 0,
            false_target: 1,
            inspect_count: 0
        };

        assert!(monkey.inspect_relieve_throw().is_none());
    }

    #[test]
    pub fn if_dividend_is_a_multiple_of_divisor_then_function_returned_by_construct_test_will_produce_true()
    {
        let divisor = 23;
        let func = construct_test(divisor);

        assert!(func.as_ref()(46));
    }

    #[test]
    pub fn if_dividend_is_not_a_multiple_of_divisor_then_function_returned_by_construct_test_will_produce_false()
    {
        let divisor = 22;
        let func = construct_test(divisor);

        assert!(!func.as_ref()(50));
    }

    #[test]
    pub fn if_construct_operation_is_presented_with_a_mul_variant_with_some_value_then_the_resulting_fn_will_multiply_by_the_variants_value()
    {
        let op = Operation::Multiply(Some(23));

        let func = construct_operation(op);

        assert_eq!(func.as_ref()(12), 12 * 23);
    }

    #[test]
    pub fn if_construct_operation_is_presented_with_a_div_variant_with_some_value_then_the_resulting_fn_will_divide_by_the_variants_value()
    {
        let op = Operation::Divide(Some(5));

        let func = construct_operation(op);

        assert_eq!(func.as_ref()(30), 6);
    }

    #[test]
    pub fn if_construct_operation_is_presented_with_a_sub_variant_with_some_value_then_the_resulting_fn_will_subtract_the_variants_value()
    {
        let op = Operation::Subtract(Some(12));
        let func = construct_operation(op);

        assert_eq!(func.as_ref()(40), 28);
    }

    #[test]
    pub fn if_construct_operation_is_presented_with_an_add_variant_with_some_value_then_the_resulting_fn_will_add_the_variants_value()
    {
        let op = Operation::Add(Some(15));
        let func = construct_operation(op);

        assert_eq!(func.as_ref()(15), 30);
    }

    #[test]
    pub fn if_construct_operation_is_presented_with_a_mul_variant_with_none_value_then_the_resulting_fn_will_square_the_input()
    {
        let op = Operation::Multiply(None);
        let func = construct_operation(op);

        assert_eq!(func.as_ref()(20), 400);
    }

    #[test]
    pub fn if_construct_operation_is_presented_with_a_div_variant_with_none_value_then_the_resulting_fn_will_always_generate_1()
    {
        let op = Operation::Divide(None);
        let func = construct_operation(op);

        assert_eq!(func.as_ref()(55), 1);
    }

    #[test]
    pub fn if_construct_operation_is_presented_with_a_sub_variant_with_none_value_then_the_resulting_fn_will_self_cancel()
    {
        let op = Operation::Subtract(None);
        let func = construct_operation(op);

        assert_eq!(func.as_ref()(234), 0);
    }

    #[test]
    pub fn if_construct_operation_is_presented_with_an_add_variant_with_none_value_then_the_resulting_fn_will_double_the_input()
    {
        let op = Operation::Add(None);
        let func = construct_operation(op);

        assert_eq!(func.as_ref()(111), 222);
    }

}