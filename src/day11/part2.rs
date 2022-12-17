use std::rc::Rc;



pub struct Monkey
{
    item_indices: Vec<usize>,
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

    pub fn add_new_monkey(&mut self, monkey_index: usize, simplifier: Rc<Box<dyn Fn(i32)->i32>>)
    {
        if monkey_index != (self.last_value.len())
        {
            panic!("Adding monkey out of order - monkey index is {}, but we have {} monkeys added now.", monkey_index, self.last_value.len())
        }

        self.last_value.push(simplifier.as_ref().as_ref()(self.start_value));
        self.simplify_with.push(simplifier);
    }

    pub fn take_action(&mut self, action: &Box<dyn Fn(i32)->i32>)
    {
        for i in 0..self.last_value.len()
        {
            self.last_value[i] = self.simplify_with[i](action.as_ref()(self.last_value[i]));
        }
    }

    pub fn last_value_for(&self, monkey_index: usize) -> i32
    {
        self.last_value[monkey_index]
    }
}


#[cfg(test)]
pub mod tests
{
    
}