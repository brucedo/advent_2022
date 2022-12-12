

pub fn solve_day_9(input: Vec<&str>)
{
    let mut instructions = Vec::<Movement>::with_capacity(input.len());
    let mut tail_moves = Vec::<(i32, i32)>::new();

    
    for line in input
    {
        instructions.push(translate_instruction_line(line));
    }

    let mut head = (0, 0);
    let mut tail = (0, 0);
    tail_moves.push(tail);

    for instruction in instructions
    {
        match instruction
        {
            Movement::Left(count) => 
            {
                tail_moves.append(&mut move_x(-1, count, &mut head, &mut tail));
            },
            Movement::Right(count) => 
            {
                tail_moves.append(&mut move_x(1, count, &mut head, &mut tail));
            },
            Movement::Up(count) => 
            {
                tail_moves.append(&mut move_y(1, count, &mut head, &mut tail));
            },
            Movement::Down(count) => 
            {
                tail_moves.append(&mut move_y(-1, count, &mut head, &mut tail));
            },
            Movement::Unknown => {},
        }
    }

    println!("The tail should have visited a total of {} spaces, some repeatedly: {:?}", tail_moves.len(), tail_moves);
    tail_moves.sort_unstable();
    tail_moves.dedup();
    println!("The tail has occupied {} unique spaces after dedup: {:?}", tail_moves.len(), tail_moves);
}

fn move_x(offset: i32, count: usize, head: &mut (i32, i32), tail: &mut(i32, i32)) -> Vec<(i32, i32)>
{
    let mut tail_moves = Vec::<(i32, i32)>::new();

    for _i in 0..count
    {
        head.0 += offset;
        if let Some(tail_offset) = adjust_tail(head, tail)
        {
            tail.0 = tail_offset.0;
            tail.1 = tail_offset.1;
            tail_moves.push((tail.0, tail.1));
        }
    }

    return tail_moves;
}

fn move_y(offset: i32, count: usize, head: &mut (i32, i32), tail: &mut(i32, i32)) -> Vec<(i32, i32)>
{
    let mut tail_moves = Vec::new();

    for _i in 0..count
    {
        head.1 += offset;
        if let Some(tail_offset) = adjust_tail(head, tail)
        {
            tail.0 = tail_offset.0;
            tail.1 = tail_offset.1;
            tail_moves.push((tail.0, tail.1));
        }
    }

    return tail_moves;
}


pub fn translate_instruction_line(line: &str) -> Movement
{
    if line.is_empty()
    {
        return Movement::Unknown
    }

    let mut split = line.split(" ");
    let direction = split.next();
    let number_str = split.next();

    if direction.is_none() || number_str.is_none()
    {
        panic!("Line format is wrong and dumb.");
    }

    let number = usize::from_str_radix(number_str.unwrap(), 10).unwrap();

    match direction.unwrap() 
    {
        "R" => {Movement::Right(number)},
        "L" => {Movement::Left(number)},
        "U" => {Movement::Up(number)},
        "D" => {Movement::Down(number)}
        _ => {Movement::Unknown}
    }
}

#[derive(PartialEq, Debug)]
pub enum Movement 
{
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
    Unknown,
}

pub fn adjust_tail(head: &mut (i32, i32), tail: &mut (i32, i32)) -> Option<(i32, i32)>
{
    let x_diff = head.0 - tail.0;
    let y_diff = head.1 - tail.1;

    if x_diff != 0 && y_diff != 0
    {
        if x_diff < -1
        {
            if y_diff < 0
            {
                return Some((tail.0 - 1, tail.1 - 1));
            }
            else if y_diff > 0
            {
                return Some((tail.0 - 1, tail.1 + 1));
            }
        }
        else if x_diff > 1
        {
            if y_diff < 0
            {
                return Some((tail.0 + 1, tail.1 - 1));
            }
            else if y_diff > 0
            {
                return Some((tail.0 + 1, tail.1 + 1));
            }
        }
        else if y_diff < -1
        {
            if x_diff < 0
            {
                return Some((tail.0 - 1, tail.1 - 1));
            }
            else if x_diff > 0
            {
                return Some((tail.0 + 1, tail.1 - 1));
            }
        }
        else if y_diff > 1
        {
            if x_diff < 0
            {
                return Some((tail.0 - 1, tail.1 + 1));
            }
            else if x_diff > 0
            {
                return Some((tail.0 + 1, tail.1 + 1));
            }
        }
        
    }
    else
    {
        if x_diff < -1
        {
            return Some((tail.0 - 1, tail.1));
        }
        else if x_diff > 1
        {
            return Some((tail.0 + 1, tail.1));
        }
        else if y_diff < -1
        {
            return Some((tail.0, tail.1 - 1));
        }
        else if y_diff > 1
        {
            return Some((tail.0, tail.1 + 1));
        }
    }
    return None;
}

#[cfg(test)]
pub mod tests
{
    use crate::day9::advent::Movement;

    use super::{adjust_tail, translate_instruction_line};

    #[test]
    pub fn when_the_input_is_r_translate_instruction_line_produces_right()
    {
        let input = "R 67";

        assert_eq!(translate_instruction_line(input), Movement::Right(67));
    }

    #[test]
    pub fn when_the_input_is_l_translate_instruction_line_produces_left()
    {
        let input = "L 43";

        assert_eq!(translate_instruction_line(input), Movement::Left(43));
    }

    #[test]
    pub fn when_the_input_is_u_translate_instruction_line_produces_up()
    {
        let input = "U 100";
        
        assert_eq!(translate_instruction_line(input), Movement::Up(100));
    }

    #[test]
    pub fn when_the_input_is_d_translate_instruction_line_produces_down()
    {
        let input = "D 23";
        assert_eq!(translate_instruction_line(input), Movement::Down(23));
    }

    #[test]
    pub fn adjust_tail_moves_tail_one_step_left_if_head_is_two_steps_left()
    {
        let mut head = (3,2);
        let mut tail = (5,2);

        assert_eq!(adjust_tail(&mut head, &mut tail), Some((4,2)));
    }

    #[test]
    pub fn adjust_tail_moves_tail_one_step_right_if_head_is_two_steps_right()
    {
        let mut head = (3,2);
        let mut tail = (1,2);

        assert_eq!(adjust_tail(&mut head, &mut tail), Some((2,2)));
    }

    #[test]
    pub fn adjust_tail_moves_tail_one_step_up_if_head_is_two_steps_up()
    {
        let mut head = (3, 5);
        let mut tail = (3, 3);

        assert_eq!(adjust_tail(&mut head, &mut tail), Some((3,4)));
    }

    #[test]
    pub fn adjust_tail_moves_tail_one_step_down_if_head_is_two_steps_down()
    {
        let mut head = (3, 5);
        let mut tail = (3, 7);

        assert_eq!(adjust_tail(&mut head, &mut tail), Some((3, 6)));

    }

    #[test]
    pub fn adjust_tail_moves_tail_up_left_if_head_is_two_left_and_one_up_or_two_up_and_one_left()
    {
        let mut head = (3, 5);
        let mut tail = (5, 4);
        assert_eq!(adjust_tail(&mut head, &mut tail), Some((4, 5)));

        let mut head = (3, 5);
        let mut tail = (4, 3);
        assert_eq!(adjust_tail(&mut head, &mut tail), Some((3, 4)));
    }

    #[test]
    pub fn adjust_tail_moves_tail_up_right_if_head_is_two_right_and_one_up_or_two_up_and_one_right()
    {
        let mut head = (7, 8);
        let mut tail = (5,7);
        assert_eq!(adjust_tail(&mut head, &mut tail), Some((6,8)));

        let mut head = (7, 8);
        let mut tail = (6, 6);
        assert_eq!(adjust_tail(&mut head, &mut tail), Some((7, 7)));
    }

    #[test]
    pub fn adjust_tail_moves_tail_down_left_if_head_is_two_left_and_one_down_or_two_down_and_one_left()
    {
        let mut head = (6, 5);
        let mut tail = (8, 6);
        assert_eq!(adjust_tail(&mut head, &mut tail), Some((7,5)));

        let mut head = (6, 5);
        let mut tail = (7,7);
        assert_eq!(adjust_tail(&mut head, &mut tail), Some((6, 6)));
    }

    #[test]
    pub fn adjust_tail_moves_tail_down_right_if_head_is_two_right_and_one_down_or_two_down_and_one_right()
    {
        let mut head = (8,6);
        let mut tail = (6,7);
        assert_eq!(adjust_tail(&mut head, &mut tail), Some((7,6)));

        let mut head = (8,6);
        let mut tail = (7,8);
        assert_eq!(adjust_tail(&mut head, &mut tail), Some((8,7)));
    }

    #[test]
    pub fn adjust_tail_returns_none_if_head_is_within_one_left_or_one_right_step_of_tail()
    {
        let mut head = (5,6);
        let mut tail = (4,6);
        assert_eq!(adjust_tail(&mut head, &mut tail), None);

        let mut head = (5,6);
        let mut tail = (6,6);
        assert_eq!(adjust_tail(&mut head, &mut tail), None);
    }

    #[test]
    pub fn adjust_tail_returns_none_if_head_is_within_one_up_or_one_down_step_of_tail()
    {
        let mut head = (5, 6);
        let mut tail = (5, 7);
        assert_eq!(adjust_tail(&mut head, &mut tail), None);

        let mut head = (5, 6);
        let mut tail = (5,5);
        assert_eq!(adjust_tail(&mut head, &mut tail), None);
    }

    #[test]
    pub fn adjust_tail_returns_none_if_head_and_tail_occupy_the_same_space()
    {
        let mut head = (5, 5);
        let mut tail = (5, 5);
        assert_eq!(adjust_tail(&mut head, &mut tail), None);
    }

    #[test]
    pub fn adjust_tail_returns_none_if_head_and_tail_are_one_space_apart_diagonally()
    {
        let mut head = (5,5);
        let mut tail = (6,6);
        assert_eq!(adjust_tail(&mut head, &mut tail), None);

        let mut tail = (6,4);
        assert_eq!(adjust_tail(&mut head, &mut tail), None);

        let mut tail = (4, 4);
        assert_eq!(adjust_tail(&mut head, &mut tail), None);

        let mut tail = (4, 6);
        assert_eq!(adjust_tail(&mut head, &mut tail), None);
    }
}