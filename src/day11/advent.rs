

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
    items: Vec<i32>,
    action: Box<dyn Fn(i32) -> i32>,
    test: Box<dyn Fn(i32) -> bool>,
    true_target: usize,
    false_target: usize,
}


#[cfg(test)]
pub mod tests
{
    use super::{Monkey, construct_operation, Operation, construct_test};

    #[test]
    pub fn after_a_monkey_tests_your_worry_level_it_will_remove_the_item_from_its_list_and_return_it_and_the_receiving_monkeys_index_as_a_tuple()
    {
        
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