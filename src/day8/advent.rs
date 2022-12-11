use log::debug;



pub fn part1(input: Vec<&str>)
{
    let grid = build_grid(break_down_lines(input));

    let mut seen_coord = Vec::<(usize, usize)>::new();

    for i in 0..grid.len()
    {
        let indices = scan_whole_line(&grid[i]);
        for index in indices
        {
            seen_coord.push((i, index));
        }
    }

    let col_count = grid[0].len();
    let mut col = Vec::<usize>::with_capacity(col_count);
    for j in 0..col_count
    {
        col.clear();
        for i in 0..grid.len()
        {
            col.push(grid[i][j]);
        }
        
        scan_whole_line(&col).iter().for_each(|index| seen_coord.push((*index, j)));
        
    }

    seen_coord.sort_unstable();
    seen_coord.dedup_by(|a, b| {debug!("{:?}, {:?} => {}", a, b, a.0 == b.0 && a.1 == b.1); a.0 == b.0 && a.1 == b.1});

    debug!("visible coords: {:?}, total count {}", seen_coord, seen_coord.len());
}

pub fn break_down_lines(lines: Vec<&str>) -> Vec<Vec<&str>>
{
    let mut grid_str = Vec::new();

    for line in lines
    {
        if line.trim().is_empty() {continue}
        
        let mut split = Vec::<&str>::new();
        for element in line.split("")
        {
            if !element.is_empty() {split.push(element);}
        }
        grid_str.push(split);
    }

    return grid_str;
}

pub fn build_grid(input: Vec<Vec<&str>>) -> Vec<Vec<usize>>
{
    let mut grid = Vec::with_capacity(input.len());

    for line in input
    {
        let mut row = Vec::<usize>::with_capacity(line.len());
        for tree_str in line
        {
            row.push(usize::from_str_radix(tree_str, 10).unwrap());
        }
        grid.push(row);
    }

    return grid;
}

pub fn search_north(grid: &Vec<Vec<usize>>, row: usize, col: usize) -> bool
{
    if row == 0
    {
        return true;
    }
    else
    {
        
        let tree_height = grid[row][col];

        for tree_line in &grid[0..row]
        {
            if tree_line[col] > tree_height { return false }
        }

        return true;
    }
}

pub fn search_east(grid: &Vec<Vec<usize>>, row: usize, col: usize) -> bool
{
    if col >= grid[0].len() - 1
    {
        return true;
    }
    else
    {
        
        let tree_height = grid[row][col];

        let tree_line = &grid[row];

        for other_tree in &tree_line[(col + 1)..grid[0].len()]
        {
            if other_tree > &tree_height { return false }
        }

        return true;
    }
}

pub fn search_south(grid: &Vec<Vec<usize>>, row: usize, col: usize) -> bool
{
    if row >= grid.len() - 1
    {
        return true;
    }
    else
    {
        
        let tree_height = grid[row][col];

        for tree_line in &grid[(row + 1)..grid.len()]
        {
            if tree_line[col] > tree_height { return false }
        }

        return true;
    }
}

pub fn search_west(grid: &Vec<Vec<usize>>, row: usize, col: usize) -> bool
{
    if col == 0
    {
        return true;
    }
    else
    {
        
        let tree_height = grid[row][col];

        let tree_line = &grid[row];

        for other_tree in &tree_line[0..col]
        {
            if *other_tree > tree_height { return false }
        }

        return true;
    }
}

pub fn scan_whole_line(line: &Vec<usize>) -> Vec<usize>
{
    let mut indices = Vec::new();
    let mut tallest = line[0];
    indices.push(0);

    for i in 1..line.len()
    {

        if line[i] > tallest
        {
            tallest = line[i];
            indices.push(i);
        }
    }

    tallest = line[line.len() - 1];
    indices.push(line.len() - 1);
    for i in (1..line.len()-1).rev()
    {
        if line[i] > tallest
        {
            tallest = line[i];
            indices.push(i);
        }
    }

    return indices;
}


#[cfg(test)]

pub mod tests
{
    use crate::day8::advent::{build_grid, search_north};

    use super::{search_east, search_west, search_south, scan_whole_line, break_down_lines};

    #[test]
    pub fn break_down_lines_will_generate_a_vec_with_as_many_rows_as_input_lines_and_whose_nested_vec_has_as_many_elements_as_each_line_has_letters()
    {
        let input = vec!["12345678", "23456789"];
        
        let result = break_down_lines(input);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 8);
        assert_eq!(result[0], vec!["1", "2", "3", "4", "5", "6", "7", "8"]);
    }

    #[test]
    pub fn scan_whole_line_will_return_a_vec_of_coordinate_pairs_of_trees_visible_from_the_east_and_west()
    {
        let mut grid = Vec::<Vec<usize>>::new();
        let row = vec![3,0,3,7,3];
        grid.push(row);
        let row = vec![2,5,5,1,2];
        grid.push(row);

        let row1_pairs = scan_whole_line(&grid[0]);
        assert_eq!(row1_pairs.len(), 4);
        assert!(row1_pairs.contains(&0));
        assert!(row1_pairs.contains(&3));
        assert!(row1_pairs.contains(&4));
        
        let row2_pairs = scan_whole_line(&grid[1]);
        assert_eq!(row2_pairs.len(), 4);
        assert!(row2_pairs.contains(&0));
        assert!(row2_pairs.contains(&1));
        assert!(row2_pairs.contains(&2));
        assert!(row2_pairs.contains(&4));
    }

    #[test]
    pub fn search_north_will_return_true_only_if_there_are_no_taller_trees_above_the_given_coordinates()
    {
        let mut grid = Vec::<Vec<usize>>::new();
        let row = vec![3,0,3,7,3];
        grid.push(row);
        let row = vec![2,5,5,1,2];
        grid.push(row);

        assert!(search_north(&grid, 1, 1));
        assert!(!search_north(&grid, 1, 0));
        assert!(search_north(&grid, 0, 3));
    }

    #[test]
    pub fn search_east_will_return_true_only_if_there_are_no_taller_trees_right_of_the_given_coordinates()
    {
        let mut grid = Vec::<Vec<usize>>::new();
        let row = vec![3,0,3,7,3];
        grid.push(row);
        let row = vec![2,5,5,1,2];
        grid.push(row);

        assert!(search_east(&grid, 0, 3));
        assert!(!search_east(&grid, 1, 3));
        assert!(search_east(&grid, 1, 4));
    }

    #[test]
    pub fn search_west_will_return_true_only_if_there_are_no_taller_trees_left_of_the_given_coordinates()
    {
        let mut grid = Vec::<Vec<usize>>::new();
        let row = vec![3,0,3,7,3];
        grid.push(row);
        let row = vec![2,5,5,1,2];
        grid.push(row);

        assert!(search_west(&grid, 1, 1));
        assert!(!search_west(&grid, 0, 1));
        assert!(search_west(&grid, 0, 0));
    }

    #[test]
    pub fn search_south_will_return_true_only_if_there_are_no_taller_trees_below_the_given_coordinates()
    {
        let mut grid = Vec::<Vec<usize>>::new();
        let row = vec![3,0,3,7,3];
        grid.push(row);
        let row = vec![2,5,5,1,2];
        grid.push(row);

        assert!(search_south(&grid, 0, 3));
        assert!(!search_south(&grid, 0, 2));
        assert!(search_south(&grid, 1, 2));
    }

    #[test]
    pub fn build_grid_creates_an_n_by_m_vec_initialized_to_0_when_given_a_vec_with_n_elements_of_vecs_of_m_str()
    {
        let mut input = Vec::<Vec<&str>>::new();
        let row = vec!["3", "0", "3", "7", "3"];
        input.push(row);
        let row = vec!["2", "5", "5", "1", "2"];
        input.push(row);
        let row = vec!["6", "5", "3", "3", "2"];
        input.push(row);
        let row = vec!["3", "3", "5", "4", "9"];
        input.push(row);
        let row = vec!["3", "5", "3", "9", "0"];
        input.push(row);

        let grid = build_grid(input);

        assert_eq!(grid.len(), 5);
        assert_eq!(grid[0].len(), 5);
        assert_eq!(grid[0], vec![3,0,3,7,3]);
        assert_eq!(grid[1], vec![2,5,5,1,2]);
        assert_eq!(grid[2], vec![6,5,3,3,2]);
        assert_eq!(grid[3], vec![3,3,5,4,9]);
        assert_eq!(grid[4], vec![3,5,3,9,0]);
    }
}