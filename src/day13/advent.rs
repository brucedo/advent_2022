

pub struct List
{

}

pub fn promote_to_list(int_value: i32) -> Box<List>
{

}

pub enum Type 
{
    
}

#[cfg(test)]
pub mod tests
{
    use crate::day13::advent::Type;

    use super::promote_to_list;

    pub fn a_lists_len_function_returns_the_count_of_scalars_plus_lists_in_the_list()
    {
        let mut list = List::new();
        list.push_scalar(0);
        list.push_scalar(12);
        list.push_scalar(14);

        let mut list2 = List::new();
        list2.push_scalar(22);
        list2.push_scalar(14);

        list.push_list(list2);

        assert_eq!(list.len(), 4);
    }

    pub fn a_lists_type_of_function_takes_an_index_and_returns_a_List_variant_if_the_index_references_a_list()
    {
        let mut list = List::new();
        list.push_scalar(23);
        
        let mut list2 = List::new();
        list2.push_scalar(45);
        list.push_list(list2);
    }

    #[test]
    pub fn passing_an_integer_to_promote_to_list_will_return_a_boxed_list_containing_the_integer()
    {
        let int_val = 16;

        let listified = promote_to_list(int_val);

        assert_eq!(listified.len(), 1);
        assert_eq!(listified.type_of(0), Type.Scalar);
        assert_eq!(listified.get_scalar(0), 16);
    }

    
}