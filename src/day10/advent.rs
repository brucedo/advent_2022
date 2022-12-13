use core::panic;
use std::collections::VecDeque;

use log::debug;



pub fn execute(program: Vec<&str>)
{
    let mut opcodes = VecDeque::<Opcode>::with_capacity(program.len());
    
    for line in program
    {
        if line.is_empty()
        {
            continue;
        }
        match decode(line)
        {
            Ok(add_codes) =>
            {
                opcodes.push_back(add_codes.0);
                opcodes.push_back(add_codes.1);
            }
            Err(noop_code) => 
            {
                opcodes.push_back(noop_code);
            }
        }
    }

    let mut reg_x = 1;
    let mut interim_x: Option<i32> = None;
    let mut samples = Vec::<i32>::new();
    let mut screen = Vec::<String>::new();
    let mut line = String::new();

    let mut cycle = 1;
    // for cycle in 1..221
    while !opcodes.is_empty()
    {
        // start of cycle
        match opcodes.pop_front() {
            Some(opcode) => 
            {
                match opcode 
                {
                    Opcode::Noop => 
                    {
                        debug!("Processing opcode this cycle ({}).", cycle);
                        interim_x = None;
                    },
                    Opcode::AddXC1 => 
                    {
                        debug!("Processing first part of add this cycle ({}).", cycle);
                        interim_x = Some(reg_x);
                    },
                    Opcode::AddXC2(value) => 
                    {
                        debug!("Processing second part of add this cycle ({}), value {}.", cycle, value);
                        interim_x = Some(reg_x + value);
                    },
                }
            },
            None => {panic!("Ran out of opcodes before end of program.");},
        }

        // Sample & draw
        if (cycle - 20) % 40 == 0
        {
            debug!("Taking sample on cycle {} with register {}, signal strength {}", cycle, reg_x, cycle * reg_x);
            samples.push(cycle * reg_x);
        }

        if ((cycle - 1) % 40) >= (reg_x - 1) && ((cycle - 1) % 40) <= (reg_x + 1)
        {
            line.push('#');
        }
        else
        {
            line.push('.');
        }

        if cycle % 40 == 0
        {
            screen.push(line);
            line = String::new();
        }

        // write phase.
        if let Some(new_x) = interim_x
        {
            reg_x = new_x;
        }
        cycle += 1;
    }

    println!("Sum of signal strengths: {}", samples.into_iter().sum::<i32>());

    println!("Screen display: ");
    screen.into_iter().for_each(|line| println!("{}", line));
}


pub fn decode(line: &str) -> Result<(Opcode, Opcode), Opcode> // This is really dumb, but it works.
{
    if line.starts_with("noop")
    {
        return Err(Opcode::Noop);
    }
    else if line.starts_with("addx")
    {
        let mut split = line.split(" ");
        split.next();
        let value = split.next().unwrap();
        return Ok((Opcode::AddXC1, Opcode::AddXC2(i32::from_str_radix(value, 10).unwrap())));
    }

    panic!("GPF: Invalid Opcode");
}


pub enum Opcode
{
    Noop,
    AddXC1,
    AddXC2(i32)
}