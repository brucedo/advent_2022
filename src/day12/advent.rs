use std::{collections::BinaryHeap, cmp::Ordering};

use log::debug;

use crate::lib::lib::char_to_num;

pub fn solver_day12(lines: Vec<&str>)
{
    let (mut map, start, end) = build_map(lines);
    let mut dists = Vec::<usize>::new();

    for row in &map
    {
        for col in row
        {
            print!("{:>3}", col.height);
        }
        println!("");
    }
    println!("{:?}", start);
    println!("{:?}", end);

    // djikstra(&mut map, start, end);

    // part 2
    let mut start_points = Vec::<(usize, usize)>::new();
    
    for row in 0..map.len()
    {
        for col in 0..map[row].len()
        {
            if map[row][col].height == 0
            {
                start_points.push((row, col))
            }
        }
    }

    for start_point in start_points
    {
        reset_map(&mut map, start_point);
        dists.push(djikstra(&mut map, start_point, end));
    }

    println!("Distances: ");
    for dist in dists
    {
        print!("{:>5}", dist);
    }
    println!();
}

fn reset_map(map: &mut Vec<Vec<Node>>, start: (usize, usize))
{

    for row in 0..map.len()
    {
        for col in 0..map[row].len()
        {
            map[row][col].previous = None;
            map[row][col].tentative_distance = u32::MAX;
            map[row][col].visited = false;
        }
    }

    map[start.0][start.1].tentative_distance = 0;
}

fn build_map(lines: Vec<&str>) -> (Vec<Vec<Node>>, (usize, usize), (usize, usize))
{
    let mut map = Vec::<Vec<Node>>::new();
    let mut start_row: usize = 0;
    let mut start_col: usize = 0;
    let mut end_row: usize = 0;
    let mut end_col: usize = 0;

    let mut curr_row: usize = 0;
    let mut curr_col: usize = 0;

    for line in lines
    {
        if line.trim().is_empty()
        {
            continue;
        }

        let mut map_row = Vec::<Node>::new();

        for char in line.chars()
        {
            let mut height: usize;

            if char == 'S'
            {
                start_row = curr_row;
                start_col = curr_col;
                height = char_to_num(&'a');
            }
            else if char == 'E'
            {
                end_row = curr_row;
                end_col = curr_col;
                height = char_to_num(&'z');
            }
            else
            {
                height = char_to_num(&char);
            }
            map_row.push(Node { distance: 1, height: height as u8, tentative_distance: u32::MAX, visited: false, previous: None });
            curr_col += 1;
        }
        map.push(map_row);
        curr_row += 1;
        curr_col = 0;
    }

    map[start_row][start_col].tentative_distance = 0;

    return (map, (start_row, start_col), (end_row, end_col));
}

fn djikstra(map: &mut Vec<Vec<Node>>, start: (usize, usize), end: (usize, usize)) -> usize
{
    let mut pqueue = BinaryHeap::<NodeOrder>::new();
    let total_distance: u32;
    pqueue.push(NodeOrder {distance: 0, index:start});

    let mut previous: Option<(usize, usize)> = None;


    loop
    {
        let next = pqueue.pop().unwrap();
        let mut index = next.index;

        debug!("Looking at index {},{}", index.0, index.1);

        while map[index.0][index.1].visited
        {
            let next = pqueue.pop().unwrap();
            index = next.index;
            debug!("Looking at index {},{}", index.0, index.1);
        }

        // neighbor indices
        let neighbors = build_neighbors(index, map);
        debug!("Neighbors: {:?}", neighbors);
        debug!("Risk from start to visited node: {}", map[index.0][index.1].tentative_distance);
        for neighbor in neighbors
        {
            if map[neighbor.0][neighbor.1].visited
            {
                continue;
            }

            let distance = map[index.0][index.1].tentative_distance + map[neighbor.0][neighbor.1].distance;
            debug!("Risk from start to neighbor {}, {}: {}", neighbor.0, neighbor.1, distance);
            map[neighbor.0][neighbor.1].tentative_distance = std::cmp::min(distance, map[neighbor.0][neighbor.1].tentative_distance);
            pqueue.push(NodeOrder{distance: map[neighbor.0][neighbor.1].tentative_distance, index: (neighbor.0, neighbor.1)});
        }

        
        map[index.0][index.1].visited = true;
        map[index.0][index.1].previous = previous;
        previous = Some((index.0, index.1));

        // if index.0 == map.len() - 1 && index.1 == map[index.0].len() - 1
        if index.0 == end.0 && index.1 == end.1
        {
            debug!("Stopping at index {}, {}", index.0, index.1);
            total_distance = map[index.0][index.1].tentative_distance;
            break;
        }

        // break;
    }

    // Trace back the path to the start
    // let mut trace_path = (map.len() - 1, map[0].len() - 1);
    let mut trace_path = (end.0, end.1);

    while trace_path != (start.0, start.1)
    {
        println!("Index: {:?}", trace_path);
        match map[trace_path.0][trace_path.1].previous
        {
            None => {println!("The path back to root is broken.  Something has gone badly wrong."); break;}
            Some(tuple) => {trace_path = tuple;}
        }
    }

    println!("Total risk: {}", total_distance);
    return total_distance as usize;
}

fn build_neighbors(index: (usize, usize), map: &Vec<Vec<Node>>) -> Vec<(usize, usize)>
{
    let mut neighbors = Vec::<(usize, usize)>::new();
    
    let curr_height = map[index.0][index.1].height;

    let previous_row = index.0.checked_sub(1);
    let next_row = index.0.checked_add(1);
    let previous_col = index.1.checked_sub(1);
    let next_col = index.1.checked_add(1);
    if !previous_row.is_none()
    {
        let up_height = map[previous_row.unwrap()][index.1].height;
        if up_height <= curr_height + 1
        {
            neighbors.push((previous_row.unwrap(), index.1));
        }
        
    }
    if !previous_col.is_none()
    {
        let left_height = map[index.0][previous_col.unwrap()].height;
        if left_height <= curr_height + 1
        {
            neighbors.push((index.0, previous_col.unwrap()))
        }
    }
    if !next_row.is_none() && next_row.unwrap() < map.len()
    {
        let down_height = map[next_row.unwrap()][index.1].height;
        if down_height <= curr_height + 1
        {
            neighbors.push((next_row.unwrap(), index.1));
        }
    }
    if !next_col.is_none() && next_col.unwrap() < map[index.0].len()
    {
        let right_height = map[index.0][next_col.unwrap()].height;
        if right_height <= curr_height + 1
        {
            neighbors.push((index.0, next_col.unwrap()));
        }
    }

    return neighbors;
}

#[derive(Debug)]
struct Node {
    distance: u32,
    height: u8,
    tentative_distance: u32,
    visited: bool,
    previous: Option<(usize, usize)>
}

struct NodeOrder {
    index: (usize, usize),
    distance: u32
}

impl Ord for NodeOrder
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for NodeOrder
{

}

impl PartialOrd for NodeOrder
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.eq(other)
        {
            Some(Ordering::Equal)
        }
        else if self.distance < other.distance
        {
            Some(Ordering::Greater)
        }
        else
        {
            Some(Ordering::Less)
        }
    }
}

impl PartialEq for NodeOrder
{

    fn eq(&self, rhs: &NodeOrder) -> bool 
    { 
        self.distance == rhs.distance
    }
}
