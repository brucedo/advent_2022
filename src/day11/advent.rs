use std::collections::VecDeque;



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
}

impl Monkey
{
    pub fn inspect_relieve_throw(&mut self) -> Option<(usize, i32)>
    {
        let next = self.items.pop_front()?;

        let post_worry = self.action.as_ref()(next);
        let post_relief = post_worry / 3;
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

    use super::{Monkey, construct_operation, Operation, construct_test};

    #[test]
    pub fn after_a_monkey_tests_your_worry_level_it_will_remove_the_item_from_its_list_and_return_it_and_the_receiving_monkeys_index_as_a_tuple()
    {
        let mut monkey0 = Monkey
        { 
            items: VecDeque::from(vec![10]), 
            action: construct_operation(Operation::Multiply(Some(9))), 
            test: construct_test(5), 
            true_target: 5, 
            false_target: 3 
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
            false_target: 2
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
            false_target: 2
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
            false_target: 1
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