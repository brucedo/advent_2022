use log::debug;


pub struct Cave
{
    grid: Vec<Vec<u8>>
}

pub fn solve_day_14_1(lines: Vec<&str>)
{
    let paths = paths(lines);

    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;

    for path in &paths
    {
        let new_maxes = find_max(path);
        max_x = std::cmp::max(new_maxes.0, max_x);
        max_y = std::cmp::max(new_maxes.1, max_y);
    }

    let mut cave = Cave::new(max_x, max_y);

    for path in &paths
    {
        cave.fill_rock_path(path);
    }

    let blocked_units = calculate_sandfall(&mut cave);

    println!("The number of units that get blocked and come to rest are {}", blocked_units);
}

fn calculate_sandfall(cave: &mut Cave) -> usize
{
    let mut sand_count = 0;
    let mut curr_pos = (500, 0);
    loop 
    {
        debug!("Current position ({},{}) vs. cave dimensions {} x {}", curr_pos.0, curr_pos.1, cave.width(), cave.depth());
        if (curr_pos.0 + 1) >= cave.width() || (curr_pos.1 + 1) >= cave.depth()
        {
            break;
        }

        if !cave.is_blocked((curr_pos.0, curr_pos.1 + 1))
        {
            curr_pos = (curr_pos.0, curr_pos.1 + 1);
        }
        else if !cave.is_blocked((curr_pos.0 - 1, curr_pos.1 + 1))
        {
            curr_pos = (curr_pos.0 - 1, curr_pos.1 + 1);
        }
        else if !cave.is_blocked((curr_pos.0 + 1, curr_pos.1 + 1))
        {
            curr_pos = (curr_pos.0 + 1, curr_pos.1 + 1);
        }
        else
        {
            cave.fill_with_sand(curr_pos);
            sand_count += 1;
            curr_pos = (500,0);
        }
    }

    return sand_count
}

fn paths(lines: Vec<&str>) -> Vec<Vec<(usize, usize)>>
{
    let mut paths = Vec::new();
    for line in lines
    {
        if line.trim().is_empty()
        {
            continue;
        }

        paths.push(pathify(line));
    }

    return paths;
}

impl Cave
{
    pub fn new(max_path_x: usize, max_path_y: usize) -> Cave
    {
        let mut grid = Vec::new();

        for _x in 0..max_path_x + 1
        {
            let mut col = Vec::<u8>::new();
            for _y in 0..max_path_y + 1
            {
                col.push(0)
            }

            grid.push(col);
        }
        
        Cave { grid }
    }

    pub fn width(&self) -> usize
    {
        self.grid.len()
    }

    pub fn depth(&self) -> usize
    {
        if self.grid.len() > 0
        {
            return self.grid.get(0).unwrap().len();
        }

        return 0;
    }

    pub fn fill_with_sand(&mut self, coord: (usize, usize))
    {
        self.grid[coord.0][coord.1] = 2;
    }

    pub fn fill_rock_path(&mut self, path: &Vec<(usize, usize)>)
    {
        let mut start: &(usize, usize) = path.get(0).unwrap();
        let mut end: &(usize, usize);

        for point in &path[1..path.len()]
        {
            end = point;

            if start.0 != end.0
            {
                self.fill_horizontal(start, end);
            }
            else if start.1 != end.1
            {
                self.fill_vertical(start, end);
            }

            start = end;
        }
    }

    pub fn is_blocked(&self, coord: (usize, usize)) -> bool
    {
        self.grid[coord.0][coord.1] != 0
    }

    fn fill_horizontal(&mut self, start: &(usize, usize), end: &(usize, usize))
    {
        let y = start.1;
        let x_start: usize;
        let x_end:usize;
        if start.0 > end.0
        {
            x_start = end.0;
            x_end = start.0;
        }
        else
        {
            x_start = start.0;
            x_end = end.0;
        }

        for i in x_start..x_end + 1
        {
            self.grid[i][y] = 1;
        }
    }

    fn fill_vertical(&mut self, start: &(usize, usize), end: &(usize, usize))
    {
        let x = start.0;
        let y_start: usize;
        let y_end: usize;

        if start.1 > end.1
        {
            y_start = end.1;
            y_end = start.1;
        }
        else
        {
            y_start = start.1;
            y_end = end.1;
        }

        for i in y_start..y_end + 1
        {
            self.grid[x][i] = 1;
        }
    }
}

pub fn pathify(line: &str) -> Vec<(usize, usize)>
{
    let mut path = Vec::<(usize, usize)>::new();
    let points = line.split(" -> ");

    for point in points
    {
        let mut split = point.split(",");
        // yes yes horribly dangerous to unwrap without checking.  I would not actually _do_ this outside of simple challenges...
        let x = usize::from_str_radix(split.next().unwrap(), 10).unwrap();
        let y = usize::from_str_radix(split.next().unwrap(), 10).unwrap();

        path.push((x,y));
    }

    return path;
}

pub fn find_max(path: &Vec<(usize, usize)>) -> (usize, usize)
{
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;

    for point in path
    {
        if point.0 > max_x { max_x = point.0; }
        if point.1 > max_y { max_y = point.1; }
    }

    return (max_x, max_y);
}

#[cfg(test)]
pub mod test
{
    use super::{find_max, pathify, Cave};


    #[test]
    pub fn filling_a_coord_with_sand_will_mark_it_as_blocked()
    {
        let mut cave = Cave::new(503, 9);

        cave.fill_with_sand((255, 5));

        assert!(cave.is_blocked((255, 5)));
    }

    #[test]
    pub fn adding_a_path_to_a_cave_with_horizontal_and_vertical_components_will_fill_all_rows_and_columns_defined_in_path()
    {
        let path: Vec<(usize, usize)> = vec![(503,4), (502,4), (502,9), (494,9)];
        let mut cave = Cave::new(503, 9);

        cave.fill_rock_path(&path);

        assert!(cave.is_blocked((503,4)));
        assert!(cave.is_blocked((502,4)));
        assert!(cave.is_blocked((502,5)));
        assert!(cave.is_blocked((502,6)));
        assert!(cave.is_blocked((502,7)));
        assert!(cave.is_blocked((502,8)));
        assert!(cave.is_blocked((502,9)));
        assert!(cave.is_blocked((501,9)));
        assert!(cave.is_blocked((500,9)));
        assert!(cave.is_blocked((499,9)));
        assert!(cave.is_blocked((498,9)));
        assert!(cave.is_blocked((497,9)));
        assert!(cave.is_blocked((496,9)));
        assert!(cave.is_blocked((495,9)));
        assert!(cave.is_blocked((494,9)));
    }

    #[test]
    pub fn adding_a_horizontal_path_to_a_cave_will_fill_a_row_on_the_path_y_axis_from_min_path_x_to_max_path_x()
    {
        let path: Vec<(usize, usize)> = vec![(498,6), (496,6)];
        let mut cave = Cave::new(498, 6);

        cave.fill_rock_path(&path);

        assert!(cave.is_blocked((498,6)));
        assert!(cave.is_blocked((497,6)));
        assert!(cave.is_blocked((496,6)));
    }

    #[test]
    pub fn adding_a_vertical_path_to_a_cave_will_fill_a_column_on_the_path_x_axis_from_min_path_y_to_max_path_y()
    {
        let path: Vec<(usize, usize)> = vec![(498,4), (498,6)];
        let mut cave = Cave::new(498, 6);

        cave.fill_rock_path(&path);

        assert!(cave.is_blocked((498,4)));
        assert!(cave.is_blocked((498,5)));
        assert!(cave.is_blocked((498,6)));
    }

    #[test]
    pub fn creating_a_cave_with_new_creates_a_grid_that_is_one_unit_wider_and_one_unit_deeper_than_inputs_ask()
    {
        let cave = Cave::new(500, 400);

        assert_eq!(cave.width(), 501);
        assert_eq!(cave.depth(), 401);
    }

    #[test]
    pub fn pathify_generates_a_vec_of_tuples_from_a_well_formatted_line()
    {
        let line1 = "498,4 -> 498,6 -> 496,6";
        let line2 = "503,4 -> 502,4 -> 502,9 -> 494,9";

        let path1 = pathify(line1);
        let path2 = pathify(line2);

        assert_eq!(path1.len(), 3);
        assert!(path1.contains(&(498,4)));
        assert!(path1.contains(&(498,6)));
        assert!(path1.contains(&(496,6)));
        assert_eq!(path2.len(), 4);
        assert!(path2.contains(&(503,4)));
        assert!(path2.contains(&(502,4)));
        assert!(path2.contains(&(502,9)));
        assert!(path2.contains(&(494,9)));
        
    }

    #[test]
    pub fn when_given_some_list_of_path_points_find_max_returns_the_largest_x_and_y_coordinates_as_a_tuple()
    {
        let path: Vec<(usize, usize)> = vec![(23, 45), (912, 2), (754, 14), (24, 999), (911, 998)];

        let max = find_max(&path);

        assert_eq!(max.0, 912);
        assert_eq!(max.1, 999);
    }
}