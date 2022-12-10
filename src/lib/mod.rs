pub mod lib
{

    pub fn find_max<T>(list: &Vec<T>) -> Option<&T>
    where T: PartialEq + PartialOrd
    {
        if list.len() == 0
        {
            return None;
        }

        let mut max: &T = list.get(0).unwrap();

        for t in list
        {
            if t.gt(max)
            {
                max = t;
            }
        }

        return Some(max);

    }

    pub fn to_lines<'a>(file_data: &'a str) -> Vec<&'a str>
    {
        let split = file_data.split("\n");
        let mut lines = Vec::<&str>::new();

        for string in split
        {
            lines.push(string.trim());
        }

        return lines;
    }

    pub fn to_untrimmed_lines<'a>(file_data: &'a str) -> Vec<&'a str>
    {
        let split = file_data.split("\n");
        // let mut lines = Vec::<&str>::new();

        return split.collect();
        // for string in split
        // {
        //     lines.push(string);
        // }

        // return lines;
    }

pub fn char_to_num(src: &char) -> usize
{
    // sigh
    match src
    {
        'a' => 0, 
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        'i' => 8,
        'j' => 9,
        'k' => 10,
        'l' => 11,
        'm' => 12,
        'n' => 13,
        'o' => 14,
        'p' => 15,
        'q' => 16,
        'r' => 17,
        's' => 18,
        't' => 19,
        'u' => 20,
        'v' => 21,
        'w' => 22,
        'x' => 23,
        'y' => 24,
        'z' => 25,
        'A' => 26,
        'B' => 27,
        'C' => 28,
        'D' => 29,
        'E' => 30,
        'F' => 31,
        'G' => 32,
        'H' => 33,
        'I' => 34,
        'J' => 35,
        'K' => 36,
        'L' => 37,
        'M' => 38,
        'N' => 39,
        'O' => 40,
        'P' => 41,
        'Q' => 42,
        'R' => 43,
        'S' => 44,
        'T' => 45,
        'U' => 46,
        'V' => 47,
        'W' => 48,
        'X' => 49,
        'Y' => 50,
        'Z' => 51,
        _ => {panic!("There should only be letters a-zA-Z here.")}
    }
}


    #[cfg(test)]
    mod test
    {
        use std::cmp::Ordering;

        use crate::lib::lib::to_lines;


        #[test]
        pub fn to_lines_produces_empty_strings_when_input_string_is_all_newlines()
        {
            let input = "\n\n\n\n\n\n\n\n\n\n\n\n";

            let lines = to_lines(input);

            assert_eq!(lines.len(), 13);
            for line in lines
            {
                assert_eq!(line.len(), 0);
            }
        }

        #[test]
        pub fn to_lines_counts_both_unix_and_windows_newlines()
        {
            let input = "\n\r\n\n\r\n\n\r\n";
            let lines = to_lines(input);

            assert_eq!(lines.len(), 7);
            for line in lines
            {
                assert_eq!(line.len(), 0);
            }
        }

        #[test]
        pub fn to_lines_produces_one_line_when_no_newline_characters_are_present()
        {
            let input = "this is a test";

            let lines = to_lines(input);
            
            assert_eq!(lines.len(), 1);
            assert_eq!(lines.get(0).unwrap().cmp(&input), Ordering::Equal);
        }
    }
}