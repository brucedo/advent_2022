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