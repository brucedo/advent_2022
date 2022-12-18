use std::{rc::Rc, collections::VecDeque};

use log::debug;

use super::advent::{Operation, construct_operation, starting_items, make_operation, decode_target, recordify};


pub fn solver(lines: Vec<&str>)
{
    let records = recordify(lines);

    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut items: Vec<Item> = Vec::new();

    let mut temp_tests = Vec::<(usize, Rc<Box<dyn Fn(i32)->i32>>)>::new();
    for record in records
    {
        temp_tests.push(construct_from_record(record, &mut monkeys, &mut items));
    }

    finish_items(&mut items, temp_tests);

    for _ in 0..10000
    {
        for i in 0..monkeys.len()
        {
            while !monkeys[i].item_indices.is_empty()
            {
                if let Some(next_item_index) = monkeys[i].item_indices.pop_front()
                {
                    items[next_item_index].take_action(&monkeys[i].action);
                    if let Some(result) = items[next_item_index].last_value_for(i)
                    {
                        let target: usize;
                        if result == &0
                        {
                            target = monkeys[i].true_target;
                        }
                        else
                        {
                            target = monkeys[i].false_target;
                        }
                        monkeys[target].item_indices.push_back(next_item_index);
                        monkeys[i].touch_count+= 1;
                    }
                }
                
            }
        }
    }

    // display loop
    println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
    println!("           STATE OF THE MONKEY UNION PASS {}", 1);

    println!("Item values: ");
    for item in items
    {
        print!("{} ", item.start_value)
    }
    println!("");
    for monkey in &monkeys
    {
        
        println!("Items and worry levels: {:?}", monkey.item_indices);
        println!("Inspection count: {}", monkey.touch_count);
    }
    println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++");

    let mut max1: usize = 0;
    let mut max2: usize = 0;

    for monkey in &monkeys
    {
        println!("Monkey touch count: {}", monkey.touch_count);
        if monkey.touch_count >= max1 { max2 = max1;  max1 = monkey.touch_count; }
        else if monkey.touch_count >= max2 { max2 = monkey.touch_count; }
    }

    println!("Busiest monkeys and total: {} * {} = {}", max1, max2, max1 * max2);
}


fn finish_items(all_items: &mut Vec<Item>, monkey_info: Vec<(usize, Rc<Box<dyn Fn(i32)->i32>>)>)
{
    for item in all_items
    {
        for (index, test) in &monkey_info
        {
            if item.add_new_monkey(*index, test.clone()).is_err()
            {
                panic!("Something went wrong while trying to load the monkey tests into the item.")
            }
        }
    }
}

pub fn construct_from_record(record: Vec<&str>, monkeys: &mut Vec<Monkey>, all_items: &mut Vec<Item>) -> (usize, Rc<Box<dyn Fn(i32)->i32>>)
{
    let mut lines = record.into_iter();

    lines.next(); // Reject monkey line
    let item_start_vals = starting_items(lines.next().unwrap());
    let action = make_operation(lines.next().unwrap());
    let test = make_test(lines.next().unwrap());
    let true_target = decode_target(lines.next().unwrap());
    let false_target = decode_target(lines.next().unwrap());

    let mut monkey = Monkey{item_indices: VecDeque::new(), action, true_target, false_target, touch_count: 0};
    
    for start_val in item_start_vals
    {
        let item_obj = Item::new(start_val);
        monkey.item_indices.push_back(all_items.len());
        all_items.push(item_obj);
    }

    monkeys.push(monkey);

    (monkeys.len() - 1, test)
}

pub fn make_test(test_line: &str) -> Rc<Box<dyn Fn(i32)->i32>>
{
    if !test_line.starts_with("Test:")
    {
        panic!("Bad input line for test builder: {}", test_line);
    }

    let divisor = i32::from_str_radix(test_line.strip_prefix("Test: divisible by ").unwrap(), 10).unwrap();

    let operation = Operation::Modulus(divisor);

    Rc::new(construct_operation(operation))
}

pub struct Monkey
{
    item_indices: VecDeque<usize>,
    action: Box<dyn Fn(i32)->i32>,
    true_target: usize,
    false_target: usize,
    touch_count: usize,
}

pub struct Item
{
    start_value: i32,
    simplify_with: Vec<Rc<Box<dyn Fn(i32)->i32>>>,
    last_value: Vec<i32>,
}

impl Item
{
    pub fn new(start_value: i32) -> Item
    {
        Item { start_value, simplify_with: Vec::new(), last_value: Vec::new() }
    }

    pub fn add_new_monkey(&mut self, monkey_index: usize, simplifier: Rc<Box<dyn Fn(i32)->i32>>) -> Result<(), ()>
    {
        if monkey_index != (self.last_value.len())
        {
            debug!("Adding monkey out of order - monkey index is {}, but we have {} monkeys added now.", monkey_index, self.last_value.len());
            return Err(());
        }

        self.last_value.push(simplifier.as_ref().as_ref()(self.start_value));
        self.simplify_with.push(simplifier);
        Ok(())
    }

    pub fn take_action(&mut self, action: &Box<dyn Fn(i32)->i32>)
    {
        for i in 0..self.last_value.len()
        {
            self.last_value[i] = self.simplify_with[i](action.as_ref()(self.last_value[i]));
        }
    }

    pub fn last_value_for(&self, monkey_index: usize) -> Option<&i32>
    {
        self.last_value.get(monkey_index)
    }
}


#[cfg(test)]
pub mod tests
{
    use std::rc::Rc;

    use super::Item;

    #[test]
    pub fn add_new_monkey_will_set_and_normalize_start_value()
    {
        let mut item = Item::new(25);
        let simplifier: Rc<Box<dyn Fn(i32)->i32>> = Rc::new(Box::new(|operand1| operand1 % 3));

        let result = item.add_new_monkey(0, simplifier);

        assert!(result.is_ok());
        assert!(item.last_value_for(0).is_some());
        assert_eq!(item.last_value_for(0).unwrap(), &1);
    }

    #[test]
    pub fn add_new_monkey_will_error_if_monkey_index_indicates_out_of_order()
    {
        let mut item = Item::new(13);
        let simplifier: Rc<Box<dyn Fn(i32)-> i32>> = Rc::new(Box::new(|operand1| operand1 % 16));

        assert_eq!(Err(()), item.add_new_monkey(1, simplifier));
    }

    #[test]
    pub fn add_new_monkey_requires_monkey_indices_to_be_monotonically_incrementing()
    {
        let mut item = Item::new(33);

        let simplifier: Rc<Box<dyn Fn(i32)->i32>> = Rc::new(Box::new(|operand| operand %9));
        let simplifier_2: Rc<Box<dyn Fn(i32)->i32>> = Rc::new(Box::new(|operand| operand %11));

        assert!(item.add_new_monkey(0, simplifier).is_ok());
        assert!(item.add_new_monkey(4, simplifier_2.clone()).is_err());
        assert!(item.add_new_monkey(1, simplifier_2).is_ok());
    }

    #[test]
    pub fn take_action_will_adjust_every_monkeys_current_value_modulus_simplifier()
    {
        let mut item = Item::new(78);

        let simplifier: Rc<Box<dyn Fn(i32)->i32>> = Rc::new(Box::new(|operand| operand %9));
        let simplifier_2: Rc<Box<dyn Fn(i32)->i32>> = Rc::new(Box::new(|operand| operand %11));

        item.add_new_monkey(0, simplifier);
        item.add_new_monkey(1, simplifier_2);

        let action: Box<dyn Fn(i32) -> i32> = Box::new(|operand| operand * 12);
        let action_2: Box<dyn Fn(i32) -> i32> = Box::new(|operand| operand + 6);

        item.take_action(&action);
        assert!(item.last_value_for(0).is_some());
        assert_eq!(item.last_value_for(0).unwrap(), &((78 * 12) % 9));
        assert!(item.last_value_for(1).is_some());
        assert_eq!(item.last_value_for(1).unwrap(), &((78 * 12) % 11));
    }
}